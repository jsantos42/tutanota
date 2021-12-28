/**
 * Hey you! Don't import anything in this file, or all these declarations will cease to be global!
 */

declare type TimeoutID = ReturnType<setTimeout>
declare type AnimationFrameID = ReturnType<requestAnimationFrame>
declare type NumberString = string
declare type Id = string
declare type IdTuple = Readonly<[string, string]>
declare type Dict = { [key: string]: string }

/** Requests from main web thread to worker */
declare type WorkerRequestType =
		| 'setup'
		// | 'generateSignupKeys'
		| 'reset'
		| 'testEcho'
		| 'testError'
		| 'restRequest'
		// | 'entityRequest'
		| 'serviceRequest'
		| 'entropy'
		| 'tryReconnectEventBus'
		| 'closeEventBus'
		| 'resolveSessionKey'
		| 'getLog'
		| 'urlify'
		| 'generateSsePushIdentifer'
		| 'facade'

/** Requests from worker web thread to main web thread */
declare type MainRequestType =
		| 'facade'
		| 'execNative'
		| 'entityEvent'
		| 'error'
		| 'progress'
		| 'updateIndexState'
		| 'updateWebSocketState'
		| 'counterUpdate'
		| 'updateLeaderStatus'
		| 'infoMessage'
		| 'createProgressMonitor'
		| 'progressWorkDone'
		| 'writeIndexerDebugLog'

/** Requests from web to native */
declare type NativeRequestType = 'init'
		| 'generateRsaKey'
		| 'rsaEncrypt'
		| 'rsaDecrypt'
		| 'aesEncryptFile'
		| 'aesDecryptFile'
		| 'open'
		| 'openFileChooser'
		| 'deleteFile'
		| 'getName'
		| 'getMimeType'
		| 'getSize'
		| 'upload'
		| 'download'
		| 'clearFileData'
		| 'findSuggestions'
		| 'initPushNotifications'
		| 'openLink'
		| 'shareText'
		| 'reload'
		| 'getPushIdentifier'
		| 'storePushIdentifierLocally'
		| 'closePushNotifications'
		| 'readDataFile'
		| 'saveBlob'
		| 'putFileIntoDownloads'
		| 'findInPage'
		| 'stopFindInPage'
		| 'registerMailto'
		| 'unregisterMailto'
		| 'openNewWindow'
		| 'setConfigValue'
		| 'enableAutoLaunch'
		| 'disableAutoLaunch'
		| 'sendSocketMessage'
		| 'getDeviceLog' // for mobile apps
		| 'getLog' // for desktop
		| 'integrateDesktop'
		| 'unIntegrateDesktop'
		| 'setSearchOverlayState'
		| 'changeLanguage'
		| 'isUpdateAvailable' // check if update is ready to install
		| 'manualUpdate' // progress update process (check, dl, install)
		| 'startNativeDrag'
		| 'mailToMsg'
		| 'focusApplicationWindow'
		| 'saveToExportDir'
		| 'checkFileExistsInExportDirectory'
		| 'scheduleAlarms'
		| 'getConfigValue'
		| 'getIntegrationInfo'
		| 'getSpellcheckLanguages'
		| 'getSelectedTheme'
		| 'setSelectedTheme'
		| 'getThemes'
		| 'setThemes'
		| 'encryptUsingKeychain'
		| 'decryptUsingKeychain'
		| 'getSupportedEncryptionModes'

/** Requests from native to web */
declare type JsRequestType = 'createMailEditor'
		| 'handleBackPress'
		| 'showAlertDialog'
		| 'openMailbox'
		| 'keyboardSizeChanged'
		| 'print'
		| 'openFindInPage'
		| 'reportError'
		| 'openCalendar'
		| 'visibilityChange'
		| 'invalidateAlarms'
		| 'applySearchResultToOverlay'
		| 'addShortcuts'
		| 'appUpdateDownloaded'
		| 'openCustomer' // only for admin clients
		| 'updateTargetUrl'
		| 'showSpellcheckDropdown'


// TODO these two should be declared under test/
declare function browser(f: Function): Function

declare function node(f: Function): Function

// see https://bitwiseshiftleft.github.io/sjcl/doc/symbols/sjcl.bitArray.html
// type that is used by sjcl for any encryption/decryption operation
// TODO these should be exported by tutanota-crypto
declare type BitArray = number[]
declare type Aes128Key = BitArray
declare type Aes256Key = BitArray
declare type SignedBytes = number[]
declare type Base32 = string