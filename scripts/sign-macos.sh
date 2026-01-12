#!/usr/bin/env -S bash -e

APP_BUNDLE_PATH="${APP_BUNDLE_PATH:?APP_BUNDLE_PATH not set}"

# 1. Create temporary keychain and import certificate (same as before)
KEYCHAIN=build.keychain-db

security create-keychain -p "$MACOS_CI_KEYCHAIN_PWD" "$KEYCHAIN"
security default-keychain -s "$KEYCHAIN"
security unlock-keychain -p "$MACOS_CI_KEYCHAIN_PWD" "$KEYCHAIN"
security set-keychain-settings "$KEYCHAIN"

echo "$MACOS_CERTIFICATE" | base64 --decode > certificate.p12
security import certificate.p12 \
  -k "$KEYCHAIN" \
  -P "$MACOS_CERTIFICATE_PWD" \
  -T /usr/bin/codesign

security set-key-partition-list -S apple-tool:,apple:,codesign: \
  -s -k "$MACOS_CI_KEYCHAIN_PWD" "$KEYCHAIN"

# 2. Sign app bundle
codesign --deep --force --options runtime --timestamp \
  --sign "$MACOS_CERTIFICATE_NAME" \
  "$APP_BUNDLE_PATH"

codesign --verify --deep --strict --verbose=2 "$APP_BUNDLE_PATH"
echo "Signed app at $APP_BUNDLE_PATH"

# 3. Notarization via App Store Connect API (recommended)
if [[ -n "$MACOS_NOTARY_KEY_ID" ]]; then
  echo "$MACOS_NOTARY_KEY" | base64 --decode > notary.key

  xcrun notarytool store-credentials "ci-profile" \
    --key notary.key \
    --key-id "$MACOS_NOTARY_KEY_ID" \
    --issuer "$MACOS_NOTARY_ISSUER_ID"

  xcrun notarytool submit "$APP_BUNDLE_PATH" \
    --key notary.key \
    --key-id "$MACOS_NOTARY_KEY_ID" \
    --issuer "$MACOS_NOTARY_ISSUER_ID" \
    --team-id "$MACOS_NOTARY_TEAM_ID" \
    --wait

  xcrun stapler staple "$APP_BUNDLE_PATH"
  echo "Notarized and stapled app."
fi
