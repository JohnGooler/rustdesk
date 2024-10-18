lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", ""),
        ("Your Desktop", ""),
        ("desk_tip", ""),
        ("Password", ""),
        ("Ready", ""),
        ("Established", ""),
        ("connecting_status", ""),
        ("Enable service", ""),
        ("Start service", ""),
        ("Service is running", ""),
        ("Service is not running", ""),
        ("not_ready_status", ""),
        ("Control Remote Desktop", ""),
        ("Transfer file", ""),
        ("Connect", ""),
        ("Recent sessions", ""),
        ("Address book", ""),
        ("Confirmation", ""),
        ("TCP tunneling", ""),
        ("Remove", ""),
        ("Refresh random password", ""),
        ("Set your own password", ""),
        ("Enable keyboard/mouse", ""),
        ("Enable clipboard", ""),
        ("Enable file transfer", ""),
        ("Enable TCP tunneling", ""),
        ("IP Whitelisting", ""),
        ("ID/Relay Server", ""),
        ("Import server config", ""),
        ("Export Server Config", ""),
        ("Import server configuration successfully", ""),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", ""),
        ("Clipboard is empty", ""),
        ("Stop service", ""),
        ("Change ID", ""),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", ""),
        ("Website", ""),
        ("About", ""),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", ""),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", ""),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive bitrate", ""),
        ("ID Server", ""),
        ("Relay Server", ""),
        ("API Server", ""),
        ("invalid_http", ""),
        ("Invalid IP", ""),
        ("Invalid format", ""),
        ("server_not_support", ""),
        ("Not available", ""),
        ("Too frequent", ""),
        ("Cancel", ""),
        ("Skip", ""),
        ("Close", ""),
        ("Retry", ""),
        ("OK", ""),
        ("Password Required", ""),
        ("Please enter your password", ""),
        ("Remember password", ""),
        ("Wrong Password", ""),
        ("Do you want to enter again?", ""),
        ("Connection Error", ""),
        ("Error", ""),
        ("Reset by the peer", ""),
        ("Connecting...", ""),
        ("Connection in progress. Please wait.", ""),
        ("Please try 1 minute later", ""),
        ("Login Error", ""),
        ("Successful", ""),
        ("Connected, waiting for image...", ""),
        ("Name", ""),
        ("Type", ""),
        ("Modified", ""),
        ("Size", ""),
        ("Show Hidden Files", ""),
        ("Receive", ""),
        ("Send", ""),
        ("Refresh File", ""),
        ("Local", ""),
        ("Remote", ""),
        ("Remote Computer", ""),
        ("Local Computer", ""),
        ("Confirm Delete", ""),
        ("Delete", ""),
        ("Properties", ""),
        ("Multi Select", ""),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", ""),
        ("Not an empty directory", ""),
        ("Are you sure you want to delete this file?", ""),
        ("Are you sure you want to delete this empty directory?", ""),
        ("Are you sure you want to delete the file of this directory?", ""),
        ("Do this for all conflicts", ""),
        ("This is irreversible!", ""),
        ("Deleting", ""),
        ("files", ""),
        ("Waiting", ""),
        ("Finished", ""),
        ("Speed", ""),
        ("Custom Image Quality", ""),
        ("Privacy mode", ""),
        ("Block user input", ""),
        ("Unblock user input", ""),
        ("Adjust Window", ""),
        ("Original", ""),
        ("Shrink", ""),
        ("Stretch", ""),
        ("Scrollbar", ""),
        ("ScrollAuto", ""),
        ("Good image quality", ""),
        ("Balanced", ""),
        ("Optimize reaction time", ""),
        ("Custom", ""),
        ("Show remote cursor", ""),
        ("Show quality monitor", ""),
        ("Disable clipboard", ""),
        ("Lock after session end", ""),
        ("Insert", ""),
        ("Insert Lock", ""),
        ("Refresh", ""),
        ("ID does not exist", ""),
        ("Failed to connect to rendezvous server", ""),
        ("Please try later", ""),
        ("Remote desktop is offline", ""),
        ("Key mismatch", ""),
        ("Timeout", ""),
        ("Failed to connect to relay server", ""),
        ("Failed to connect via rendezvous server", ""),
        ("Failed to connect via relay server", ""),
        ("Failed to make direct connection to remote desktop", ""),
        ("Set Password", ""),
        ("OS Password", ""),
        ("install_tip", ""),
        ("Click to upgrade", ""),
        ("Click to download", ""),
        ("Click to update", ""),
        ("Configure", ""),
        ("config_acc", ""),
        ("config_screen", ""),
        ("Installing ...", ""),
        ("Install", ""),
        ("Installation", ""),
        ("Installation Path", ""),
        ("Create start menu shortcuts", ""),
        ("Create desktop icon", ""),
        ("agreement_tip", ""),
        ("Accept and Install", ""),
        ("End-user license agreement", ""),
        ("Generating ...", ""),
        ("Your installation is lower version.", ""),
        ("not_close_tcp_tip", ""),
        ("Listening ...", ""),
        ("Remote Host", ""),
        ("Remote Port", ""),
        ("Action", ""),
        ("Add", ""),
        ("Local Port", ""),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", ""),
        ("Too short, at least 6 characters.", ""),
        ("The confirmation is not identical.", ""),
        ("Permissions", ""),
        ("Accept", ""),
        ("Dismiss", ""),
        ("Disconnect", ""),
        ("Enable file copy and paste", ""),
        ("Connected", ""),
        ("Direct and encrypted connection", ""),
        ("Relayed and encrypted connection", ""),
        ("Direct and unencrypted connection", ""),
        ("Relayed and unencrypted connection", ""),
        ("Enter Remote ID", ""),
        ("Enter your password", ""),
        ("Logging in...", ""),
        ("Enable RDP session sharing", ""),
        ("Auto Login", ""),
        ("Enable direct IP access", ""),
        ("Rename", ""),
        ("Space", ""),
        ("Create desktop shortcut", ""),
        ("Change Path", ""),
        ("Create Folder", ""),
        ("Please enter the folder name", ""),
        ("Fix it", ""),
        ("Warning", ""),
        ("Login screen using Wayland is not supported", ""),
        ("Reboot required", ""),
        ("Unsupported display server", ""),
        ("x11 expected", ""),
        ("Port", ""),
        ("Settings", ""),
        ("Username", ""),
        ("Invalid port", ""),
        ("Closed manually by the peer", ""),
        ("Enable remote configuration modification", ""),
        ("Run without install", ""),
        ("Connect via relay", ""),
        ("Always connect via relay", ""),
        ("whitelist_tip", ""),
        ("Login", ""),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", ""),
        ("Tags", ""),
        ("Search ID", ""),
        ("whitelist_sep", ""),
        ("Add ID", ""),
        ("Add Tag", ""),
        ("Unselect all tags", ""),
        ("Network error", ""),
        ("Username missed", ""),
        ("Password missed", ""),
        ("Wrong credentials", ""),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", ""),
        ("Forget Password", ""),
        ("Favorites", ""),
        ("Add to Favorites", ""),
        ("Remove from Favorites", ""),
        ("Empty", ""),
        ("Invalid folder name", ""),
        ("Socks5 Proxy", ""),
        ("Socks5/Http(s) Proxy", ""),
        ("Discovered", ""),
        ("install_daemon_tip", ""),
        ("Remote ID", ""),
        ("Paste", ""),
        ("Paste here?", ""),
        ("Are you sure to close the connection?", ""),
        ("Download new version", ""),
        ("Touch mode", ""),
        ("Mouse mode", ""),
        ("One-Finger Tap", ""),
        ("Left Mouse", ""),
        ("One-Long Tap", ""),
        ("Two-Finger Tap", ""),
        ("Right Mouse", ""),
        ("One-Finger Move", ""),
        ("Double Tap & Move", ""),
        ("Mouse Drag", ""),
        ("Three-Finger vertically", ""),
        ("Mouse Wheel", ""),
        ("Two-Finger Move", ""),
        ("Canvas Move", ""),
        ("Pinch to Zoom", ""),
        ("Canvas Zoom", ""),
        ("Reset canvas", ""),
        ("No permission of file transfer", ""),
        ("Note", ""),
        ("Connection", ""),
        ("Share Screen", ""),
        ("Chat", ""),
        ("Total", ""),
        ("items", ""),
        ("Selected", ""),
        ("Screen Capture", ""),
        ("Input Control", ""),
        ("Audio Capture", ""),
        ("File Connection", ""),
        ("Screen Connection", ""),
        ("Do you accept?", ""),
        ("Open System Setting", ""),
        ("How to get Android input permission?", ""),
        ("android_input_permission_tip1", ""),
        ("android_input_permission_tip2", ""),
        ("android_new_connection_tip", ""),
        ("android_service_will_start_tip", ""),
        ("android_stop_service_tip", ""),
        ("android_version_audio_tip", ""),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", ""),
        ("Overwrite", ""),
        ("This file exists, skip or overwrite this file?", ""),
        ("Quit", ""),
        ("Help", ""),
        ("Failed", ""),
        ("Succeeded", ""),
        ("Someone turns on privacy mode, exit", ""),
        ("Unsupported", ""),
        ("Peer denied", ""),
        ("Please install plugins", ""),
        ("Peer exit", ""),
        ("Failed to turn off", ""),
        ("Turned off", ""),
        ("Language", ""),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Enable remote restart", ""),
        ("Restart remote device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting remote device", ""),
        ("remote_restarting_tip", ""),
        ("Copied", ""),
        ("Exit Fullscreen", ""),
        ("Fullscreen", ""),
        ("Mobile Actions", ""),
        ("Select Monitor", ""),
        ("Control Actions", ""),
        ("Display Settings", ""),
        ("Ratio", ""),
        ("Image Quality", ""),
        ("Scroll Style", ""),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", ""),
        ("Relay Connection", ""),
        ("Secure Connection", ""),
        ("Insecure Connection", ""),
        ("Scale original", ""),
        ("Scale adaptive", ""),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Light Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable recording session", ""),
        ("Enable LAN discovery", ""),
        ("Deny LAN discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", ""),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", ""),
        ("JumpLink", ""),
        ("Please Select the screen to be shared(Operate on the peer side).", ""),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to address book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
        ("Follow remote cursor", ""),
        ("Follow remote window focus", ""),
        ("default_proxy_tip", ""),
        ("no_audio_input_device_tip", ""),
        ("Incoming", ""),
        ("Outgoing", ""),
        ("Clear Wayland screen selection", ""),
        ("clear_Wayland_screen_selection_tip", ""),
        ("confirm_clear_Wayland_screen_selection_tip", ""),
        ("android_new_voice_call_tip", ""),
        ("texture_render_tip", ""),
        ("Use texture rendering", ""),
        ("Floating window", ""),
        ("floating_window_tip", ""),
        ("Keep screen on", ""),
        ("Never", ""),
        ("During controlled", ""),
        ("During service is on", ""),
        ("Capture screen using DirectX", ""),
        ("Back", ""),
        ("Apps", ""),
        ("Volume up", ""),
        ("Volume down", ""),
        ("Power", ""),
        ("Telegram bot", ""),
        ("enable-bot-tip", ""),
        ("enable-bot-desc", ""),
        ("cancel-2fa-confirm-tip", ""),
        ("cancel-bot-confirm-tip", ""),
        ("About RustDesk", ""),
        ("Send clipboard keystrokes", ""),
        ("network_error_tip", ""),
        ("Unlock with PIN", ""),
        ("Requires at least {} characters", ""),
        ("Wrong PIN", ""),
        ("Set PIN", ""),
        ("Enable trusted devices", ""),
        ("Manage trusted devices", ""),
        ("Platform", ""),
        ("Days remaining", ""),
        ("enable-trusted-devices-tip", ""),
        ("Parent directory", ""),
        ("Resume", ""),
        ("Invalid file name", ""),
        ("one-way-file-transfer-tip", ""),
        ("Authentication Required", ""),
        ("Authenticate", ""),
        ("web_id_input_tip", ""),
        ("Download", ""),
        ("Upload folder", ""),
        ("Upload files", ""),
        ("Clipboard is synchronized", ""),
    ].iter().cloned().collect();
}
