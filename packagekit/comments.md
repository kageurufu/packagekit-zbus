<!-- file * -->
<!-- fn bitfield_contain_priority -->
Finds elements in a list, but with priority going to the preceeding entry
## `values`
a valid bitfield instance
## `value`
the first value we are searching for

# Returns

The return enumerated type, or -1 if none are found
<!-- fn bitfield_from_enums -->
Create a bitfield with the suppied values set.
## `value`
the first value we want to add to the bitfield

# Returns

a `PkBitfield`, or 0 if invalid
<!-- fn debug_get_option_group -->
Returns a `GOptionGroup` for the commandline arguments recognized
by debugging. You should add this group to your `GOptionContext`
with `g_option_context_add_group()`, if you are using
`g_option_context_parse()` to parse your commandline arguments.

# Returns

a `GOptionGroup` for the commandline arguments
<!-- fn enum_find_string -->
Search for a enum value in a table of constants.
## `table`
A `PkEnumMatch` enum table of values
## `value`
the enumerated constant value, e.g. PK_SIGTYPE_ENUM_GPG

# Returns

the string constant, e.g. "desktop-gnome"
<!-- fn enum_find_value -->
Search for a string value in a table of constants.
## `table`
A `PkEnumMatch` enum table of values
## `string`
the string constant to search for, e.g. "desktop-gnome"

# Returns

the enumerated constant value, e.g. PK_SIGTYPE_ENUM_GPG
<!-- static DBUS_INTERFACE -->
The DBUS interface used by the PackageKit service.
<!-- static DBUS_INTERFACE_OFFLINE -->
The DBUS interface for PackageKit offline update functionality
<!-- static DBUS_INTERFACE_TRANSACTION -->
The DBUS interface for PackageKit transactions.
<!-- static DBUS_PATH -->
The DBUS path to the PackageKit service.
<!-- static DBUS_SERVICE -->
The DBUS name for the PackageKit system service.
<!-- static DESKTOP_DEFAULT_APPLICATION_DIR -->
The default location for the desktop files
<!-- const PACKAGE_ID_ARCH -->
Alias to get an arch field from the result of pk_package_id_split
<!-- const PACKAGE_ID_DATA -->
Alias to get a data field from the result of pk_package_id_split
<!-- const PACKAGE_ID_NAME -->
Alias to get a name field from the result of pk_package_id_split
<!-- const PACKAGE_ID_VERSION -->
Alias to get a version field from the result of pk_package_id_split
<!-- enum AuthorizeEnum::variant Unknown -->
Unknown authorization status
<!-- enum AuthorizeEnum::variant Yes -->
Authorized
<!-- enum AuthorizeEnum::variant No -->
Not authorized
<!-- enum AuthorizeEnum::variant Interactive -->
Interaction required for authorization
<!-- struct Category -->


## Properties


#### `cat-id`
 Readable | Writeable


#### `icon`
 Readable | Writeable


#### `name`
 Readable | Writeable


#### `parent-id`
 Readable | Writeable


#### `summary`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`CategoryExt`][trait@crate::prelude::CategoryExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct Client -->


## Properties


#### `background`
 Readable | Writeable


#### `cache-age`
 Readable | Writeable


#### `idle`
 Readable


#### `interactive`
 Readable | Writeable


#### `locale`
 Readable | Writeable

# Implements

[`ClientExt`][trait@crate::prelude::ClientExt]
<!-- trait ClientExt::fn accept_eula -->
We may want to agree to a EULA dialog if one is presented.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `eula_id`
the `<literal>`eula_id`</literal>` we are agreeing to
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn accept_eula_async -->
We may want to agree to a EULA dialog if one is presented.
## `eula_id`
the `<literal>`eula_id`</literal>` we are agreeing to
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn adopt -->
Adopt a transaction.

Warning: this function is synchronous, and will block. Do not use it in GUI
applications.
## `transaction_id`
a transaction ID such as "/21_ebcbdaae_data"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn adopt_async -->
Adopt a transaction which allows the caller to monitor the state or cancel it.
## `transaction_id`
a transaction ID such as "/21_ebcbdaae_data"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn depends_on -->
Get the packages that depend this one, i.e. child.parent.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
If we should search recursively for depends
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn depends_on_async -->
Get the packages that depend this one, i.e. child->parent.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
If we should search recursively for depends
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn download_packages -->
Downloads package files to a specified location.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `directory`
the location where packages are to be downloaded
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn download_packages_async -->
Downloads package files to a specified location.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `directory`
the location where packages are to be downloaded
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn categories -->
Get a list of all categories supported.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn categories_async -->
Get a list of all categories supported.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn details -->
Get details of a package, so more information can be obtained for GUI
or command line tools.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn details_async -->
Get details of a package, so more information can be obtained for GUI
or command line tools.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn details_local -->
Get details of a local package, so more information can be obtained for GUI
or command line tools.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `files`
a null terminated array of filenames
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn details_local_async -->
Get details of a package, so more information can be obtained for GUI
or command line tools.
## `files`
a null terminated array of filenames
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn distro_upgrades -->
This method should return a list of distribution upgrades that are available.
It should not return updates, only major upgrades.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn distro_upgrades_async -->
This method should return a list of distribution upgrades that are available.
It should not return updates, only major upgrades.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn files -->
Get the file list (i.e. a list of files installed) for the specified package.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn files_async -->
Get the file list (i.e. a list of files installed) for the specified package.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn files_local -->
Get file list of a local package, so more information can be obtained for GUI
or command line tools.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `files`
a null terminated array of filenames
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn files_local_async -->
Get file list of a package, so more information can be obtained for GUI
or command line tools.
## `files`
a null terminated array of filenames
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn old_transactions -->
Get the old transaction list, mainly used for the transaction viewer.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `number`
the number of past transactions to return, or 0 for all
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn old_transactions_async -->
Get the old transaction list, mainly used for the transaction viewer.
## `number`
the number of past transactions to return, or 0 for all
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn packages -->
Get the list of packages from the backend

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn packages_async -->
Get the list of packages from the backend
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn progress_async -->
Find the current state of a transaction.
## `transaction_id`
a transaction ID such as "/21_ebcbdaae_data"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn repo_list -->
Get the list of repositories installed on the system.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Development`][crate::FilterEnum::Development] or [`FilterEnum::None`][crate::FilterEnum::None]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn repo_list_async -->
Get the list of repositories installed on the system.
## `filters`
a `PkBitfield` such as [`FilterEnum::Development`][crate::FilterEnum::Development] or [`FilterEnum::None`][crate::FilterEnum::None]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn update_detail -->
Get details about the specific update, for instance any CVE urls and
severity information.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn update_detail_async -->
Get details about the specific update, for instance any CVE urls and
severity information.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn updates -->
Get a list of all the packages that can be updated for all repositories.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Development`][crate::FilterEnum::Development] or [`FilterEnum::None`][crate::FilterEnum::None]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn updates_async -->
Get a list of all the packages that can be updated for all repositories.
## `filters`
a `PkBitfield` such as [`FilterEnum::Development`][crate::FilterEnum::Development] or [`FilterEnum::None`][crate::FilterEnum::None]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn install_files -->
Install a file locally, and get the deps from the repositories.
This is useful for double clicking on a .rpm or .deb file.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
a transaction type bitfield
## `files`
a file such as "/home/hughsie/Desktop/hal-devel-0.10.0.rpm"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn install_files_async -->
Install a file locally, and get the deps from the repositories.
This is useful for double clicking on a .rpm or .deb file.
## `transaction_flags`
a transaction type bitfield
## `files`
a file such as "/home/hughsie/Desktop/hal-devel-0.10.0.rpm"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn install_packages -->
Install a package of the newest and most correct version.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
a transaction type bitfield
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn install_packages_async -->
Install a package of the newest and most correct version.
## `transaction_flags`
a transaction type bitfield
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn install_signature -->
Install a software repository signature of the newest and most correct version.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `type_`
the signature type, e.g. [`SigTypeEnum::Gpg`][crate::SigTypeEnum::Gpg]
## `key_id`
a key ID such as "0df23df"
## `package_id`
a signature_id structure such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn install_signature_async -->
Install a software repository signature of the newest and most correct version.
## `type_`
the signature type, e.g. [`SigTypeEnum::Gpg`][crate::SigTypeEnum::Gpg]
## `key_id`
a key ID such as "0df23df"
## `package_id`
a signature_id structure such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn refresh_cache -->
Refresh the cache, i.e. download new metadata from a remote URL so that
package lists are up to date.
This action may take a few minutes and should be done when the session and
system are idle.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `force`
if we should aggressively drop caches
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn refresh_cache_async -->
Refresh the cache, i.e. download new metadata from a remote URL so that
package lists are up to date.
This action may take a few minutes and should be done when the session and
system are idle.
## `force`
if we should aggressively drop caches
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn remove_packages -->
Remove a package (optionally with dependancies) from the system.
If `allow_deps` is set to [`false`], and other packages would have to be removed,
then the transaction would fail.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
a transaction type bitfield
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `allow_deps`
if other dependent packages are allowed to be removed from the computer
## `autoremove`
if other packages installed at the same time should be tried to remove
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn remove_packages_async -->
Remove a package (optionally with dependancies) from the system.
If `allow_deps` is set to [`false`], and other packages would have to be removed,
then the transaction would fail.
## `transaction_flags`
a transaction type bitfield
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `allow_deps`
if other dependent packages are allowed to be removed from the computer
## `autoremove`
if other packages installed at the same time should be tried to remove
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn repair_system -->
This transaction will try to recover from a broken package management system:
e.g. the installation of a package with unsatisfied dependencies has
been forced by using a low level tool (rpm or dpkg) or the
system was shutdown during processing an installation.

The backend will decide what is best to do.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
if only trusted packages should be installed
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn repair_system_async -->
This transaction will try to recover from a broken package management system:
e.g. the installation of a package with unsatisfied dependencies has
been forced by the user using a low level tool (rpm or dpkg) or the
system was shutdown during processing an installation.

The backend will decide what is best to do.
## `transaction_flags`
a transaction type bitfield
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn repo_enable -->
Enable or disable the repository.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `repo_id`
a repo_id structure such as "livna-devel"
## `enabled`
if we should enable the repository
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn repo_enable_async -->
Enable or disable the repository.
## `repo_id`
a repo_id structure such as "livna-devel"
## `enabled`
if we should enable the repository
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn repo_remove -->
Removes a repo and optionally the packages installed from it.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
transaction flags
## `repo_id`
a repo_id structure such as "livna-devel"
## `autoremove`
If packages should be auto-removed
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn repo_remove_async -->
Removes a repo and optionally the packages installed from it.
## `transaction_flags`
transaction flags
## `repo_id`
a repo_id structure such as "livna-devel"
## `autoremove`
If packages should be auto-removed
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn repo_set_data -->
We may want to set a repository parameter.
NOTE: this is free text, and is left to the backend to define a format.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `repo_id`
a repo_id structure such as "livna-devel"
## `parameter`
the parameter to change
## `value`
what we should change it to
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn repo_set_data_async -->
We may want to set a repository parameter.
NOTE: this is free text, and is left to the backend to define a format.
## `repo_id`
a repo_id structure such as "livna-devel"
## `parameter`
the parameter to change
## `value`
what we should change it to
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn required_by -->
Get the packages that require this one, i.e. parent.child.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
If we should search recursively for requires
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn required_by_async -->
Get the packages that require this one, i.e. parent->child.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
If we should search recursively for requires
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn resolve -->
Resolve a package name into a `package_id`. This can return installed and
available packages and allows you find out if a package is installed locally
or is available in a repository.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `packages`
an array of package names to resolve, e.g. "gnome-system-tools"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn resolve_async -->
Resolve a package name into a `package_id`. This can return installed and
available packages and allows you find out if a package is installed locally
or is available in a repository.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `packages`
an array of package names to resolve, e.g. "gnome-system-tools"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn search_details -->
Search all detailed summary information to try and find a keyword.
Think of this as `pk_client_search_names()`, but trying much harder and
taking longer.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
free text to search for, for instance, "power"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn search_details_async -->
Search all detailed summary information to try and find a keyword.
Think of this as `pk_client_search_names()`, but trying much harder and
taking longer.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
free text to search for, for instance, "power"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn search_files -->
Search for packages that provide a specific file.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
file to search for, for instance, "/sbin/service"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn search_files_async -->
Search for packages that provide a specific file.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
file to search for, for instance, "/sbin/service"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn search_groups -->
Return all packages in a specific group.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
a group enum to search for, for instance, "system-tools"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn search_groups_async -->
Return all packages in a specific group.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
a group enum to search for, for instance, "system-tools"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn search_names -->
Search all the locally installed files and remote repositories for a package
that matches a specific name.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
free text to search for, for instance, "power"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn search_names_async -->
Search all the locally installed files and remote repositories for a package
that matches a specific name.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
free text to search for, for instance, "power"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn update_packages -->
Update specific packages to the newest available versions.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
a transaction type bitfield
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn update_packages_async -->
Update specific packages to the newest available versions.
## `transaction_flags`
a transaction type bitfield
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn upgrade_system -->
This transaction will upgrade the distro to the next version, which may
involve just downloading the installer and setting up the boot device,
or may involve doing an on-line upgrade.

The backend will decide what is best to do.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `transaction_flags`
transaction flags
## `distro_id`
a distro ID such as "fedora-14"
## `upgrade_kind`
a [`UpgradeKindEnum`][crate::UpgradeKindEnum] such as [`UpgradeKindEnum::Complete`][crate::UpgradeKindEnum::Complete]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn upgrade_system_async -->
This transaction will update the distro to the next version, which may
involve just downloading the installer and setting up the boot device,
or may involve doing an on-line upgrade.

The backend will decide what is best to do.
## `transaction_flags`
a transaction type bitfield
## `distro_id`
a distro ID such as "fedora-14"
## `upgrade_kind`
a [`UpgradeKindEnum`][crate::UpgradeKindEnum] such as [`UpgradeKindEnum::Complete`][crate::UpgradeKindEnum::Complete]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait ClientExt::fn what_provides -->
This should return packages that provide the supplied attributes.
This method is useful for finding out what package(s) provide a modalias
or GStreamer codec string.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
a search term such as "sound/mp3"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait ClientExt::fn what_provides_async -->
This should return packages that provide the supplied attributes.
This method is useful for finding out what package(s) provide a modalias
or GStreamer codec string.
## `filters`
a `PkBitfield` such as [`FilterEnum::Gui`][crate::FilterEnum::Gui] | [`FilterEnum::Free`][crate::FilterEnum::Free] or [`FilterEnum::None`][crate::FilterEnum::None]
## `values`
a search term such as "sound/mp3"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- enum ClientError::variant Failed -->
the transaction failed for an unknown reason
<!-- enum ClientError::variant FailedAuth -->
the transaction failed authentication/authorization
<!-- enum ClientError::variant NoTid -->
the transaction id was not pre-allocated (internal error)
<!-- enum ClientError::variant AlreadyTid -->
the transaction id has already been used (internal error)
<!-- enum ClientError::variant RoleUnknown -->
the role was not set (internal error)
<!-- enum ClientError::variant CannotStartDaemon -->
the PackageKit daemon failed to start
<!-- enum ClientError::variant InvalidInput -->
the package_id is invalid
<!-- enum ClientError::variant InvalidFile -->
the file is invalid
<!-- enum ClientError::variant NotSupported -->
the action is not supported
<!-- enum ClientError::variant DeclinedSimulation -->
the simulation was declined by the user
<!-- enum ClientError::variant DeclinedInteraction -->
the user declined interaction on the task
<!-- struct Control -->


## Properties


#### `backend-author`
 Readable | Writeable


#### `backend-description`
 Readable | Writeable


#### `backend-name`
 Readable | Writeable


#### `connected`
 Readable | Writeable


#### `distro-id`
 Readable | Writeable


#### `filters`
 Readable | Writeable


#### `groups`
 Readable | Writeable


#### `locked`
 Readable | Writeable


#### `mime-types`
 Readable | Writeable


#### `network-state`
 Readable | Writeable


#### `provides`
 Readable | Writeable


#### `roles`
 Readable | Writeable


#### `version-major`
 Readable


#### `version-micro`
 Readable


#### `version-minor`
 Readable

## Signals


#### `repo-list-changed`
 The ::repo-list-changed signal is emitted when the repo list may have
changed and the control program may have to update some UI.




#### `restart-schedule`
 The ::restart_schedule signal is emitted when the packagekitd service
has been restarted because it has been upgraded.
Client programs should reload themselves when it is convenient to
do so, as old client tools may not be compatable with the new daemon.




#### `transaction-list-changed`
 The ::transaction-list-changed signal is emitted when the list
of transactions handled by the daemon is changed.




#### `updates-changed`
 The ::updates-changed signal is emitted when the update list may have
changed and the control program may have to update some UI.



# Implements

[`ControlExt`][trait@crate::prelude::ControlExt]
<!-- trait ControlExt::fn set_proxy2_async -->
Set a proxy on the PK daemon
## `proxy_http`
a HTTP proxy string such as "username:password`server`:8080", or [`None`]
## `proxy_https`
a HTTPS proxy string such as "username:password`server`:8080", or [`None`]
## `proxy_ftp`
a FTP proxy string such as "server.lan:8080", or [`None`]
## `proxy_socks`
a SOCKS proxy string such as "server.lan:8080", or [`None`]
## `no_proxy`
a list of download IPs that shouldn't go through the proxy, or [`None`]
## `pac`
a PAC string, or [`None`]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `callback`
the function to run on completion
<!-- trait ControlExt::fn connect_transaction_list_changed -->
The ::transaction-list-changed signal is emitted when the list
of transactions handled by the daemon is changed.
## `transaction_ids`
an `GStrv` array of transaction ID's
<!-- enum ControlError::variant Failed -->
the transaction failed for an unknown reason
<!-- enum ControlError::variant CannotStartDaemon -->
the PackageKit daemon failed to start
<!-- struct Desktop -->


# Implements

[`DesktopExt`][trait@crate::prelude::DesktopExt]
<!-- struct Details -->


## Properties


#### `description`
 Readable | Writeable


#### `download-size`
 Readable | Writeable


#### `group`
 Readable | Writeable


#### `license`
 Readable | Writeable


#### `package-id`
 Readable | Writeable


#### `size`
 Readable | Writeable


#### `summary`
 Readable | Writeable


#### `url`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`DetailsExt`][trait@crate::prelude::DetailsExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct DistroUpgrade -->


## Properties


#### `name`
 Readable | Writeable


#### `state`
 Readable | Writeable


#### `summary`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`DistroUpgradeExt`][trait@crate::prelude::DistroUpgradeExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum DistroUpgradeEnum::variant Unknown -->
Unknown disto upgrade state
<!-- enum DistroUpgradeEnum::variant Stable -->
Upgraded to stable release
<!-- enum DistroUpgradeEnum::variant Unstable -->
Upgraded to unstable release
<!-- struct Error -->


## Properties


#### `code`
 Readable | Writeable


#### `details`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`ErrorExt`][trait@crate::prelude::ErrorExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum ErrorEnum::variant Oom -->
Out of memory
<!-- enum ErrorEnum::variant NoNetwork -->
No network access available
<!-- enum ErrorEnum::variant NotSupported -->
Request not supported
<!-- enum ErrorEnum::variant InternalError -->
Undefined internal error
<!-- enum ErrorEnum::variant GpgFailure -->
GPG encryption failure
<!-- enum ErrorEnum::variant PackageIdInvalid -->
Invalid package ID provided
<!-- enum ErrorEnum::variant PackageNotInstalled -->
Requested package not installed
<!-- enum ErrorEnum::variant PackageNotFound -->
Requested package not found
<!-- enum ErrorEnum::variant PackageAlreadyInstalled -->
Requested package already installed
<!-- enum ErrorEnum::variant PackageDownloadFailed -->
Failed to download package
<!-- enum ErrorEnum::variant GroupNotFound -->
Requested group not gound
<!-- enum ErrorEnum::variant GroupListInvalid -->
Invalid group list provided
<!-- enum ErrorEnum::variant DepResolutionFailed -->
Failed to resolve dependencies
<!-- enum ErrorEnum::variant FilterInvalid -->
Invalid filter provides
<!-- enum ErrorEnum::variant CreateThreadFailed -->
Failed to create thread
<!-- enum ErrorEnum::variant TransactionError -->
Error occurred during transaction
<!-- enum ErrorEnum::variant TransactionCancelled -->
Transaction was cancelled
<!-- enum ErrorEnum::variant NoCache -->
No cache available
<!-- enum ErrorEnum::variant RepoNotFound -->
Requested repository not found
<!-- enum ErrorEnum::variant CannotRemoveSystemPackage -->
Not allowed to remove system package
<!-- enum ErrorEnum::variant ProcessKill -->
Process killed
<!-- enum ErrorEnum::variant FailedConfigParsing -->
Configuration is not valid
<!-- enum ErrorEnum::variant CannotGetLock -->
Cannot get lock
<!-- enum ErrorEnum::variant NoPackagesToUpdate -->
No packages to update
<!-- enum ErrorEnum::variant CannotWriteRepoConfig -->
Cannot write repository configuration
<!-- enum ErrorEnum::variant BadGpgSignature -->
Bad GPG signature found
<!-- enum ErrorEnum::variant MissingGpgSignature -->
Required GPG signature not found
<!-- enum ErrorEnum::variant CannotInstallSourcePackage -->
Cannot install source package
<!-- enum ErrorEnum::variant FileConflicts -->
File conflicts detected
<!-- enum ErrorEnum::variant PackageConflicts -->
Package conflict
<!-- enum ErrorEnum::variant RepoNotAvailable -->
Repository not available
<!-- enum ErrorEnum::variant PackageInstallBlocked -->
Package installation blocked
<!-- enum ErrorEnum::variant PackageCorrupt -->
Package corruption occurred
<!-- enum ErrorEnum::variant AllPackagesAlreadyInstalled -->
All packages already installed
<!-- enum ErrorEnum::variant FileNotFound -->
Required file not found
<!-- enum ErrorEnum::variant NoMoreMirrorsToTry -->
Out of repository mirrors to try
<!-- enum ErrorEnum::variant NoDistroUpgradeData -->
No distribution upgrade path found
<!-- enum ErrorEnum::variant IncompatibleArchitecture -->
Incompatible architecture found
<!-- enum ErrorEnum::variant NoSpaceOnDevice -->
Out of required disk space
<!-- enum ErrorEnum::variant MediaChangeRequired -->
Need to change media
<!-- enum ErrorEnum::variant NotAuthorized -->
Authorization failed
<!-- enum ErrorEnum::variant UpdateNotFound -->
Update not found
<!-- enum ErrorEnum::variant CannotInstallRepoUnsigned -->
Installation repository missing signature
<!-- enum ErrorEnum::variant CannotUpdateRepoUnsigned -->
Update repository missing signature
<!-- enum ErrorEnum::variant CannotGetFilelist -->
Cannot get file list
<!-- enum ErrorEnum::variant CannotGetRequires -->
Cannot get package requirements
<!-- enum ErrorEnum::variant CannotDisableRepository -->
Cannot disable reposoitory
<!-- enum ErrorEnum::variant PackageFailedToConfigure -->
Package failed to configure
<!-- enum ErrorEnum::variant PackageFailedToBuild -->
Package failed to build
<!-- enum ErrorEnum::variant PackageFailedToInstall -->
Package failed to install
<!-- enum ErrorEnum::variant PackageFailedToRemove -->
Package failed to remove
<!-- enum ErrorEnum::variant InstallRootInvalid -->
Installtion root not suitable
<!-- enum ErrorEnum::variant CannotFetchSources -->
Cannot fetch sources
<!-- enum ErrorEnum::variant CancelledPriority -->
Cancelled due to higher priority task
<!-- enum ErrorEnum::variant UnfinishedTransaction -->
Transaction unfinished
<!-- enum ErrorEnum::variant LockRequired -->
Required lock not available
<!-- struct EulaRequired -->


## Properties


#### `eula-id`
 ID for this EULA.

Readable | Writeable


#### `license-agreement`
 The text of the license agreement.

Readable | Writeable


#### `package-id`
 PackageID this EULA is for.

Readable | Writeable


#### `vendor-name`
 Vendor this EULA is from.

Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`EulaRequiredExt`][trait@crate::prelude::EulaRequiredExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum ExitEnum::variant Unknown -->
Unknown exit status
<!-- enum ExitEnum::variant Success -->
Backend exited successfully
<!-- enum ExitEnum::variant Failed -->
Backend failed
<!-- enum ExitEnum::variant Cancelled -->
Backend was cancelled
<!-- enum ExitEnum::variant KeyRequired -->
A repository encryption key needs installing
<!-- enum ExitEnum::variant EulaRequired -->
A EULA is required to be accepted
<!-- enum ExitEnum::variant Killed -->
Backend was killed
<!-- enum ExitEnum::variant MediaChangeRequired -->
Media change required
<!-- enum ExitEnum::variant CancelledPriority -->
Cancelled due to higher priority task
<!-- enum ExitEnum::variant RepairRequired -->
Package database requires repairing
<!-- struct Files -->


## Properties


#### `files`
 Readable | Writeable


#### `package-id`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`FilesExt`][trait@crate::prelude::FilesExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum FilterEnum::variant Unknown -->
Unknown filter
<!-- enum FilterEnum::variant None -->
No filter
<!-- enum FilterEnum::variant Installed -->
Filter for installed packages
<!-- enum FilterEnum::variant NotInstalled -->
Filter for not installed packages
<!-- enum FilterEnum::variant Development -->
Filter for development packages
<!-- enum FilterEnum::variant NotDevelopment -->
Filter for non-development packages
<!-- enum FilterEnum::variant Gui -->
Filter for GUI packages
<!-- enum FilterEnum::variant NotGui -->
Filter for non-GUI packages
<!-- enum FilterEnum::variant Free -->
Filter for free packages
<!-- enum FilterEnum::variant NotFree -->
Filter for non-free packages
<!-- enum FilterEnum::variant Visible -->
Filter for visible packages
<!-- enum FilterEnum::variant NotVisible -->
Filter for invisible packages
<!-- enum FilterEnum::variant Supported -->
Filter for supported packages
<!-- enum FilterEnum::variant NotSupported -->
Filter for not supported packages
<!-- enum FilterEnum::variant Basename -->
Filter for packages that match basename
<!-- enum FilterEnum::variant NotBasename -->
Filter for packages that don't match basename
<!-- enum FilterEnum::variant Newest -->
Filter for newest packages
<!-- enum FilterEnum::variant NotNewest -->
Filter for not newest packages
<!-- enum FilterEnum::variant Arch -->
Filter for packages that match architecture
<!-- enum FilterEnum::variant NotArch -->
Filter for packages that don't match architecture
<!-- enum FilterEnum::variant Source -->
Filter for source packages
<!-- enum FilterEnum::variant NotSource -->
Filter for non-source packages
<!-- enum FilterEnum::variant Collections -->
Filter for collections
<!-- enum FilterEnum::variant NotCollections -->
Filter for not collections
<!-- enum FilterEnum::variant Application -->
Filter for application packages
<!-- enum FilterEnum::variant NotApplication -->
Filter for non-application packages
<!-- enum FilterEnum::variant Downloaded -->
Filter for downloaded packages
<!-- enum FilterEnum::variant NotDownloaded -->
Filter for not downloaded packages
<!-- enum GroupEnum::variant Unknown -->
Unknown group
<!-- enum GroupEnum::variant Accessibility -->
Accessibility related packages
<!-- enum GroupEnum::variant Accessories -->
Accessory packages
<!-- enum GroupEnum::variant AdminTools -->
Administration tools packages
<!-- enum GroupEnum::variant Communication -->
Communication packages
<!-- enum GroupEnum::variant DesktopGnome -->
GNOME packages
<!-- enum GroupEnum::variant DesktopKde -->
KDE packages
<!-- enum GroupEnum::variant DesktopOther -->
Other desktop packages
<!-- enum GroupEnum::variant DesktopXfce -->
XFCE packages
<!-- enum GroupEnum::variant Education -->
Education packages
<!-- enum GroupEnum::variant Fonts -->
Fonts
<!-- enum GroupEnum::variant Games -->
Games
<!-- enum GroupEnum::variant Graphics -->
Graphics related packages
<!-- enum GroupEnum::variant Internet -->
Internet related packages
<!-- enum GroupEnum::variant Legacy -->
Legacy packages
<!-- enum GroupEnum::variant Localization -->
Localization related packages
<!-- enum GroupEnum::variant Maps -->
Map related packages
<!-- enum GroupEnum::variant Multimedia -->
Multimedia packages
<!-- enum GroupEnum::variant Network -->
Network related packages
<!-- enum GroupEnum::variant Office -->
Office packages
<!-- enum GroupEnum::variant PowerManagement -->
Power-management related packages
<!-- enum GroupEnum::variant Programming -->
Programming packages
<!-- enum GroupEnum::variant Publishing -->
Publishing related packages
<!-- enum GroupEnum::variant Security -->
Security packages
<!-- enum GroupEnum::variant Servers -->
Server related packages
<!-- enum GroupEnum::variant System -->
System packages
<!-- enum GroupEnum::variant Virtualization -->
Virtualization packages
<!-- enum GroupEnum::variant Science -->
Science related packages
<!-- enum GroupEnum::variant Documentation -->
Documentation
<!-- enum GroupEnum::variant Electronics -->
Electronics package
<!-- enum GroupEnum::variant Vendor -->
Vendor defined group
<!-- enum GroupEnum::variant Newest -->
Special group for recently updated packages
<!-- enum InfoEnum::variant Unknown -->
Package status is unknown
<!-- enum InfoEnum::variant Installed -->
Package is installed
<!-- enum InfoEnum::variant Available -->
Package is available to be installed
<!-- enum InfoEnum::variant Blocked -->
Package is blocked
<!-- enum InfoEnum::variant Downloading -->
Package is downloading
<!-- enum InfoEnum::variant Updating -->
Package is updating
<!-- enum InfoEnum::variant Installing -->
Package is being installed
<!-- enum InfoEnum::variant Removing -->
Package is being removed
<!-- enum InfoEnum::variant Cleanup -->
Package is running cleanup
<!-- enum InfoEnum::variant Reinstalling -->
Package is being reinstalled
<!-- enum InfoEnum::variant Downgrading -->
Package is being downgraded
<!-- enum InfoEnum::variant Preparing -->
Package is preparing for installation/removal
<!-- enum InfoEnum::variant Decompressing -->
Package is decompressing
<!-- enum InfoEnum::variant Unavailable -->
Package is unavailable
<!-- enum InfoEnum::variant Critical -->
Update severity is critical; Since: 1.2.4
<!-- struct ItemProgress -->


## Properties


#### `package-id`
 Readable | Writeable


#### `percentage`
 Readable | Writeable


#### `status`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`ItemProgressExt`][trait@crate::prelude::ItemProgressExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct MediaChangeRequired -->


## Properties


#### `media-id`
 Readable | Writeable


#### `media-text`
 Readable | Writeable


#### `media-type`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`MediaChangeRequiredExt`][trait@crate::prelude::MediaChangeRequiredExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum MediaTypeEnum::variant Unknown -->
Unknown media type
<!-- enum MediaTypeEnum::variant Cd -->
Media is a CD
<!-- enum MediaTypeEnum::variant Dvd -->
Media is a DVD
<!-- enum MediaTypeEnum::variant Disc -->
Media is a disc (not CD or DVD)
<!-- enum NetworkEnum::variant Unknown -->
Unknown network
<!-- enum NetworkEnum::variant Offline -->
Offline (no network)
<!-- enum NetworkEnum::variant Online -->
Online (network type unknown)
<!-- enum NetworkEnum::variant Wired -->
Wired network
<!-- enum NetworkEnum::variant Wifi -->
WiFi network
<!-- enum NetworkEnum::variant Mobile -->
Mobile network
<!-- enum OfflineAction::variant Unknown -->
Unknown
<!-- enum OfflineAction::variant Reboot -->
Reboot
<!-- enum OfflineAction::variant PowerOff -->
Power-off
<!-- enum OfflineAction::variant Unset -->
No action set
<!-- enum OfflineError::variant Failed -->
No specific reason
<!-- enum OfflineError::variant InvalidValue -->
An invalid value was specified
<!-- enum OfflineError::variant NoData -->
No data was available
<!-- struct OfflineFlags -->
Flags to be used for the method invocations.
<!-- struct OfflineFlags::const NONE -->
No specific flag
<!-- struct OfflineFlags::const INTERACTIVE -->
Run the action in an interactive mode, allowing polkit authentication dialogs
<!-- struct Package -->


## Properties


#### `description`
 Readable | Writeable


#### `group`
 Readable | Writeable


#### `info`
 Readable | Writeable


#### `license`
 Readable | Writeable


#### `package-id`
 Readable


#### `size`
 Readable | Writeable


#### `summary`
 Readable | Writeable


#### `update-bugzilla-urls`
 Readable | Writeable


#### `update-changelog`
 Readable | Writeable


#### `update-cve-urls`
 Readable | Writeable


#### `update-issued`
 Readable | Writeable


#### `update-obsoletes`
 Readable | Writeable


#### `update-restart`
 Readable | Writeable


#### `update-severity`
 Can be one of [`InfoEnum::Unknown`][crate::InfoEnum::Unknown], [`InfoEnum::Low`][crate::InfoEnum::Low], [`InfoEnum::Normal`][crate::InfoEnum::Normal],
[`InfoEnum::Important`][crate::InfoEnum::Important] or [`InfoEnum::Critical`][crate::InfoEnum::Critical].

Readable | Writeable


#### `update-state`
 Readable | Writeable


#### `update-text`
 Readable | Writeable


#### `update-updated`
 Readable | Writeable


#### `update-updates`
 Readable | Writeable


#### `update-vendor-urls`
 Readable | Writeable


#### `url`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

## Signals


#### `changed`
 The ::changed signal is emitted when the package data may have changed.



# Implements

[`PackageExt`][trait@crate::prelude::PackageExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct PackageSack -->


# Implements

[`PackageSackExt`][trait@crate::prelude::PackageSackExt]
<!-- trait PackageSackExt::fn details_async -->
Merges in details about packages.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback`
the function to run on completion
<!-- trait PackageSackExt::fn update_detail_async -->
Merges in update details about packages.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback`
the function to run on completion
<!-- trait PackageSackExt::fn resolve_async -->
Merges in details about packages using resolve.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback`
the function to run on completion
<!-- enum PackageSackSortType::variant Name -->
Sort by name
<!-- enum PackageSackSortType::variant Info -->
Sort by package info
<!-- enum PackageSackSortType::variant PackageId -->
Sort by package ID
<!-- enum PackageSackSortType::variant Summary -->
Sort by summary
<!-- struct Progress -->


## Properties


#### `allow-cancel`
 [`true`] if this transaction can be cancelled.

Readable | Writeable


#### `caller-active`
 [`true`] if the transaction caller is still connected.

Readable | Writeable


#### `download-size-remaining`
 Number of bytes remaining to download.

Readable | Writeable


#### `elapsed-time`
 Amount of time the transaction has taken in seconds.

Readable | Writeable


#### `item-progress`
 Item progress associated with this transaction.

Readable | Writeable


#### `package`
 The package this transaction is acting on.

Readable | Writeable


#### `package-id`
 Package ID this transaction is acting on.

Readable | Writeable


#### `percentage`
 Percentage complete of this transaction.

Readable | Writeable


#### `remaining-time`
 Amount of time the transaction will take to complete in seconds or 0 if unknown.

Readable | Writeable


#### `role`
 Role of this transaction.

Readable | Writeable


#### `speed`
 Transaction speed in bits per second or 0 if unknown.

Readable | Writeable


#### `status`
 Status of this transaction.

Readable | Writeable


#### `transaction-flags`
 A `PkBitfield` containing [`TransactionFlagEnum`][crate::TransactionFlagEnum] associated with this transaction.

Readable | Writeable


#### `transaction-id`
 ID used by this transaction.

Readable | Writeable


#### `uid`
 The UID that started this transaction.

Readable | Writeable

# Implements

[`ProgressExt`][trait@crate::prelude::ProgressExt]
<!-- struct RepoDetail -->


## Properties


#### `description`
 Readable | Writeable


#### `enabled`
 Readable | Writeable


#### `repo-id`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`RepoDetailExt`][trait@crate::prelude::RepoDetailExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct RepoSignatureRequired -->


## Properties


#### `key-fingerprint`
 Readable | Writeable


#### `key-id`
 Readable | Writeable


#### `key-timestamp`
 Readable | Writeable


#### `key-url`
 Readable | Writeable


#### `key-userid`
 Readable | Writeable


#### `package-id`
 Readable | Writeable


#### `repository-name`
 Readable | Writeable


#### `type`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`RepoSignatureRequiredExt`][trait@crate::prelude::RepoSignatureRequiredExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct RequireRestart -->


## Properties


#### `package-id`
 Readable | Writeable


#### `restart`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`RequireRestartExt`][trait@crate::prelude::RequireRestartExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum RestartEnum::variant Unknown -->
Unknown restart state
<!-- enum RestartEnum::variant None -->
No restart required
<!-- enum RestartEnum::variant Application -->
Need to restart the application
<!-- enum RestartEnum::variant Session -->
Need to restart the session
<!-- enum RestartEnum::variant System -->
Need to restart the system
<!-- struct Results -->


## Properties


#### `inputs`
 Readable | Writeable


#### `progress`
 Readable | Writeable


#### `role`
 The [`RoleEnum`][crate::RoleEnum] or [`RoleEnum::Unknown`][crate::RoleEnum::Unknown] if not set

Readable | Writeable


#### `transaction-flags`
 A `PkBitfield` containing [`TransactionFlagEnum`][crate::TransactionFlagEnum] for this result.

Readable | Writeable

# Implements

[`ResultsExt`][trait@crate::prelude::ResultsExt]
<!-- enum RoleEnum::variant Unknown -->
Unknow request
<!-- enum RoleEnum::variant Cancel -->
Cancel transaction
<!-- enum RoleEnum::variant DependsOn -->
Get package dependencies
<!-- enum RoleEnum::variant GetDetails -->
Get package details
<!-- enum RoleEnum::variant GetPackages -->
Get available packages
<!-- enum RoleEnum::variant GetRepoList -->
Get repository list
<!-- enum RoleEnum::variant RequiredBy -->
Get packages required by given package
<!-- enum RoleEnum::variant GetUpdateDetail -->
Get update details
<!-- enum RoleEnum::variant GetUpdates -->
Get available updates
<!-- enum RoleEnum::variant InstallFiles -->
Install package files
<!-- enum RoleEnum::variant InstallPackages -->
Install packages
<!-- enum RoleEnum::variant InstallSignature -->
Install signature
<!-- enum RoleEnum::variant RefreshCache -->
Refresh cache
<!-- enum RoleEnum::variant RemovePackages -->
Remove packages
<!-- enum RoleEnum::variant RepoEnable -->
Enable repository
<!-- enum RoleEnum::variant Resolve -->
Resolve depdencies
<!-- enum RoleEnum::variant SearchDetails -->
Search for details
<!-- enum RoleEnum::variant SearchFile -->
Search for file
<!-- enum RoleEnum::variant SearchGroup -->
Search for group
<!-- enum RoleEnum::variant SearchName -->
Search for package name
<!-- enum RoleEnum::variant UpdatePackages -->
Update packages
<!-- enum RoleEnum::variant WhatProvides -->
Get what a package provides
<!-- enum RoleEnum::variant AcceptEula -->
Accept an EULA
<!-- enum RoleEnum::variant DownloadPackages -->
Download packages
<!-- enum RoleEnum::variant GetDistroUpgrades -->
Get available distribution upgrades
<!-- enum RoleEnum::variant GetCategories -->
Get available categories
<!-- enum RoleEnum::variant GetOldTransactions -->
Get old transation information
<!-- enum RoleEnum::variant RepairSystem -->
Repair system
<!-- enum RoleEnum::variant GetDetailsLocal -->
Get details on local package
<!-- enum RoleEnum::variant GetFilesLocal -->
Get files provided by local package
<!-- enum RoleEnum::variant RepoRemove -->
Remove repository
<!-- enum RoleEnum::variant UpgradeSystem -->
Upgrade system
<!-- enum SigTypeEnum::variant Unknown -->
Unkwown signature type
<!-- enum SigTypeEnum::variant Gpg -->
GPG signature
<!-- struct Source -->


## Properties


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable

# Implements

[`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum StatusEnum::variant Unknown -->
Unknown status
<!-- enum StatusEnum::variant Wait -->
Waiting
<!-- enum StatusEnum::variant Setup -->
Setting up
<!-- enum StatusEnum::variant Running -->
Running
<!-- enum StatusEnum::variant Remove -->
Removing
<!-- enum StatusEnum::variant RefreshCache -->
Refreshing cache
<!-- enum StatusEnum::variant Download -->
Downloading
<!-- enum StatusEnum::variant Install -->
Installing
<!-- enum StatusEnum::variant Update -->
Updating
<!-- enum StatusEnum::variant Cleanup -->
Cleaning up
<!-- enum StatusEnum::variant DepResolve -->
Resolving dependencies
<!-- enum StatusEnum::variant SigCheck -->
Checking signatures
<!-- enum StatusEnum::variant TestCommit -->
Testing commit
<!-- enum StatusEnum::variant Commit -->
Committing
<!-- enum StatusEnum::variant Finished -->
Finished
<!-- enum StatusEnum::variant Cancel -->
Cancelling
<!-- enum StatusEnum::variant DownloadRepository -->
Downloading respository
<!-- enum StatusEnum::variant DownloadPackagelist -->
Donwloading package list
<!-- enum StatusEnum::variant DownloadFilelist -->
Downloading file list
<!-- enum StatusEnum::variant DownloadChangelog -->
Downloading changelog information
<!-- enum StatusEnum::variant DownloadGroup -->
Downloading group information
<!-- enum StatusEnum::variant DownloadUpdateinfo -->
Downloading update information
<!-- enum StatusEnum::variant Repackaging -->
Repackaging
<!-- enum StatusEnum::variant LoadingCache -->
Loading cache
<!-- enum StatusEnum::variant ScanApplications -->
Scanning for applications
<!-- enum StatusEnum::variant GeneratePackageList -->
Generating package list
<!-- enum StatusEnum::variant WaitingForLock -->
Waiting for lock
<!-- enum StatusEnum::variant WaitingForAuth -->
Waiting for authentication/authorization
<!-- enum StatusEnum::variant ScanProcessList -->
Scanning running processes
<!-- enum StatusEnum::variant CheckExecutableFiles -->
Checking executable files
<!-- enum StatusEnum::variant CheckLibraries -->
Checking libraries
<!-- enum StatusEnum::variant CopyFiles -->
Copying files
<!-- enum StatusEnum::variant RunHook -->
Running package hook
<!-- struct Task -->


## Properties


#### `allow-downgrade`
 [`true`] if package downgrades are allowed.

Readable | Writeable


#### `allow-reinstall`
 [`true`] if package reinstallation shall be allowed during transaction.

Readable | Writeable


#### `only-download`
 [`true`] if we are just preparing the transaction for later.

Readable | Writeable


#### `only-trusted`
 [`true`] if only authenticated packages should be allowed in the transaction.

Readable | Writeable


#### `simulate`
 [`true`] if we are simulating.

Readable | Writeable
<details><summary><h4>Client</h4></summary>


#### `background`
 Readable | Writeable


#### `cache-age`
 Readable | Writeable


#### `idle`
 Readable


#### `interactive`
 Readable | Writeable


#### `locale`
 Readable | Writeable
</details>

# Implements

[`TaskExt`][trait@crate::prelude::TaskExt], [`ClientExt`][trait@crate::prelude::ClientExt]
<!-- trait TaskExt::fn depends_on_async -->
Get the list of dependant packages.
## `filters`
a bitfield of filters that can be used to limit the results
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
if we should recurse to packages that depend on other packages
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn depends_on_sync -->
Get the list of dependent packages.
## `filters`
a bitfield of filters that can be used to limit the results
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
if we should recurse to packages that depend on other packages
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn download_packages_async -->
Downloads packages
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `directory`
the destination directory
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn download_packages_sync -->
Downloads packages
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `directory`
the destination directory
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn categories_async -->
Get the categories available.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn categories_sync -->
Get the categories available.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn details_async -->
Gets details about packages.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn details_sync -->
Gets details about packages.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn files_async -->
Get the files in a package.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn files_sync -->
Get the files in a package.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn packages_async -->
Gets the list of packages.
## `filters`
a bitfield of filters that can be used to limit the results
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn packages_sync -->
Gets the list of packages.
## `filters`
a bitfield of filters that can be used to limit the results
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn repo_list_async -->
Get the list of available repositories.
## `filters`
a bitfield of filters that can be used to limit the results
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn repo_list_sync -->
Get the list of available repositories.
## `filters`
a bitfield of filters that can be used to limit the results
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn update_detail_async -->
Gets details about updates.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn update_detail_sync -->
Gets details about updates.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn updates_async -->
Gets the update lists.
## `filters`
a bitfield of filters that can be used to limit the results
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn updates_sync -->
Gets the update lists.
## `filters`
a bitfield of filters that can be used to limit the results
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn install_files_async -->
Install a file locally, and get the deps from the repositories.
This is useful for double clicking on a .rpm or .deb file.
## `files`
a file such as "/home/hughsie/Desktop/hal-devel-0.10.0.rpm"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn install_files_sync -->
Install a file locally, and get the deps from the repositories.
This is useful for double clicking on a .rpm or .deb file.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `files`
a file such as "/home/hughsie/Desktop/hal-devel-0.10.0.rpm"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn install_packages_async -->
Merges in details about packages using resolve.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn install_packages_sync -->
Install a package of the newest and most correct version.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn refresh_cache_async -->
Refresh the package cache.
## `force`
if the metadata should be deleted and re-downloaded even if it is correct
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn refresh_cache_sync -->
Refresh the package cache.
## `force`
if the metadata should be deleted and re-downloaded even if it is correct
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn remove_packages_async -->
Remove a package (optionally with dependancies) from the system.
If `allow_deps` is set to [`false`], and other packages would have to be removed,
then the transaction would fail.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `allow_deps`
if other dependent packages are allowed to be removed from the computer
## `autoremove`
if other packages installed at the same time should be tried to remove
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn remove_packages_sync -->
Remove a package (optionally with dependancies) from the system.
If `allow_deps` is set to [`false`], and other packages would have to be removed,
then the transaction would fail.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `allow_deps`
if other dependent packages are allowed to be removed from the computer
## `autoremove`
if other packages installed at the same time should be tried to remove
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn repair_system_async -->
Recover the system from broken dependencies and aborted installations.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn repair_system_sync -->
Recover from broken dependencies of installed packages or incomplete
installations.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn repo_enable_async -->
Enable or disable a specific repo.
## `repo_id`
The software repository ID
## `enabled`
[`true`] or [`false`]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn repo_enable_sync -->
Enable or disable a specific repo.
## `repo_id`
The software repository ID
## `enabled`
[`true`] or [`false`]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn required_by_async -->
Get the packages this package requires.
## `filters`
a bitfield of filters that can be used to limit the results
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
if we should return packages that depend on the ones we do
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn required_by_sync -->
Get the packages this package requires.
## `filters`
a bitfield of filters that can be used to limit the results
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `recursive`
if we should return packages that depend on the ones we do
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn resolve_async -->
Resolves a package name to a package-id.
## `filters`
a bitfield of filters that can be used to limit the results
## `packages`
package names to find
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn resolve_sync -->
Resolves a package name to a package-id.
## `filters`
a bitfield of filters that can be used to limit the results
## `packages`
package names to find
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn search_details_async -->
Searches for some package details.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn search_details_sync -->
Searches for some package details.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn search_files_async -->
Searches for specific files.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn search_files_sync -->
Searches for specific files.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn search_groups_async -->
Searches the group lists.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn search_groups_sync -->
Searches the group lists.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn search_names_async -->
Searches for a package name.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn search_names_sync -->
Searches for a package name.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
search values
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn update_packages_async -->
Update specific packages to the newest available versions.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn update_packages_sync -->
Update specific packages to the newest available versions.

Warning: this function is synchronous, and may block. Do not use it in GUI
applications.
## `package_ids`
a null terminated array of package_id structures such as "hal;0.0.1;i386;fedora"
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn upgrade_system_async -->
This transaction will update the distro to the next version, which may
involve just downloading the installer and setting up the boot device,
or may involve doing an on-line upgrade.

The backend will decide what is best to do.
## `distro_id`
a distro ID such as "fedora-14"
## `upgrade_kind`
a [`UpgradeKindEnum`][crate::UpgradeKindEnum] such as [`UpgradeKindEnum::Complete`][crate::UpgradeKindEnum::Complete]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn upgrade_system_sync -->
This transaction will update the distro to the next version, which may
involve just downloading the installer and setting up the boot device,
or may involve doing an on-line upgrade.

The backend will decide what is best to do.
## `distro_id`
a distro ID such as "fedora-14"
## `upgrade_kind`
a [`UpgradeKindEnum`][crate::UpgradeKindEnum] such as [`UpgradeKindEnum::Complete`][crate::UpgradeKindEnum::Complete]
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn what_provides_async -->
Find the package that provides some resource.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
values to search for
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`
## `callback_ready`
the function to run on completion
<!-- trait TaskExt::fn what_provides_sync -->
Find the package that provides some resource.
## `filters`
a bitfield of filters that can be used to limit the results
## `values`
values to search for
## `cancellable`
a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
## `progress_callback`
the function to run when the progress changes
## `progress_user_data`
data to pass to `progress_callback`

# Returns

a [`Results`][crate::Results] object, or [`None`] for error
<!-- trait TaskExt::fn allow_downgrade -->
[`true`] if package downgrades are allowed.
<!-- trait TaskExt::fn allow_reinstall -->
[`true`] if package reinstallation shall be allowed during transaction.
<!-- trait TaskExt::fn only_download -->
[`true`] if we are just preparing the transaction for later.
<!-- trait TaskExt::fn only_trusted -->
[`true`] if only authenticated packages should be allowed in the transaction.
<!-- enum TransactionFlagEnum::variant None -->
No transaction flag
<!-- enum TransactionFlagEnum::variant OnlyTrusted -->
Only allow trusted packages
<!-- enum TransactionFlagEnum::variant Simulate -->
Simulate transaction
<!-- enum TransactionFlagEnum::variant OnlyDownload -->
Only download packages
<!-- enum TransactionFlagEnum::variant AllowReinstall -->
Allow package reinstallation
<!-- enum TransactionFlagEnum::variant JustReinstall -->
Only allow package reinstallation
<!-- enum TransactionFlagEnum::variant AllowDowngrade -->
Allow packages to be downgraded
<!-- struct TransactionList -->


## Signals


#### `added`
 The ::added signal is emitted when a tid has been added to the transaction list




#### `removed`
 The ::removed signal is emitted when a tid has been removed from the transaction list



# Implements

[`TransactionListExt`][trait@crate::prelude::TransactionListExt]
<!-- struct TransactionPast -->


## Properties


#### `cmdline`
 Readable | Writeable


#### `data`
 Readable | Writeable


#### `duration`
 Readable | Writeable


#### `role`
 Readable | Writeable


#### `succeeded`
 Readable | Writeable


#### `tid`
 Readable | Writeable


#### `timespec`
 Readable | Writeable


#### `uid`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`TransactionPastExt`][trait@crate::prelude::TransactionPastExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- struct UpdateDetail -->


## Properties


#### `bugzilla-urls`
 Readable | Writeable


#### `changelog`
 Readable | Writeable


#### `cve-urls`
 Readable | Writeable


#### `issued`
 Readable | Writeable


#### `obsoletes`
 Readable | Writeable


#### `package-id`
 Readable | Writeable


#### `restart`
 Readable | Writeable


#### `state`
 Readable | Writeable


#### `update-text`
 Readable | Writeable


#### `updated`
 Readable | Writeable


#### `updates`
 Readable | Writeable


#### `vendor-urls`
 Readable | Writeable
<details><summary><h4>Source</h4></summary>


#### `role`
 Readable | Writeable


#### `transaction-id`
 Readable | Writeable
</details>

# Implements

[`UpdateDetailExt`][trait@crate::prelude::UpdateDetailExt], [`SourceExt`][trait@crate::prelude::SourceExt]
<!-- enum UpdateStateEnum::variant Unknown -->
Update stability unknown
<!-- enum UpdateStateEnum::variant Stable -->
Update is a stable release
<!-- enum UpdateStateEnum::variant Unstable -->
Update is an unstable release
<!-- enum UpdateStateEnum::variant Testing -->
Update is a testing release
<!-- enum UpgradeKindEnum::variant Minimal -->
Perform minimal upgrade
<!-- enum UpgradeKindEnum::variant Default -->
Perform default upgrade
<!-- enum UpgradeKindEnum::variant Complete -->
Perform complete upgrade
