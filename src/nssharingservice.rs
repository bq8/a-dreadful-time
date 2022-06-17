use cocoa::appkit::{NSApp, NSApplication};
use cocoa::base::{BOOL, id, nil};
use cocoa::foundation::{NSArray, NSAutoreleasePool, NSString};
use objc::runtime::{Class, Object};

#[link(name = "AppKit", kind = "framework")]
extern {
	pub static NSSharingServiceNameComposeEmail: id;
}

// https://github.com/kattrali/rust-mac-app-examples
pub unsafe fn open_email_compose_window(message: String) {
	let _pool = NSAutoreleasePool::new(nil);
	let app = NSApp();

	println!("xyz inside unsafe fn");
	let cls = Class::get("NSSharingService").unwrap();
	println!("xyz unwrapped; sending msg");
	let sharing_svc: *mut Object = msg_send![cls, sharingServiceNamed:NSSharingServiceNameComposeEmail];
	// let foo: *mut NSString = msg_send![sharing_svc, description];
	let item = NSString::alloc(nil).init_str(&message);
	let items = NSArray::arrayWithObject(nil, item);
	let can_perform: BOOL = msg_send![sharing_svc, canPerformWithItems:items];
	println!("xyz can_perform: {}", can_perform);
	let _: *mut Object = msg_send![sharing_svc, performWithItems:items];

	// FIXME: Why does it only work when we call run here:
	app.run();
}
