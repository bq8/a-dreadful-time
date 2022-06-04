//
//  AppDelegate.m
//  OGSharingService
//
//  Created by Brian Quach on 2022/06/03.
//

#import "AppDelegate.h"

@interface AppDelegate () <NSSharingServiceDelegate>
@end

@implementation AppDelegate

- (void)applicationDidFinishLaunching:(NSNotification *)notification {
    NSSharingService *sharingService = [NSSharingService sharingServiceNamed:NSSharingServiceNameComposeEmail];
    sharingService.delegate = self;
    [sharingService performWithItems:@[@"Foo"]];
}

- (void)applicationWillTerminate:(NSNotification *)notification {
    // NOOP
}

#pragma mark NSSharingServiceDelegate

- (void)sharingService:(NSSharingService *)sharingService didShareItems:(NSArray *)items {
    exit(EXIT_SUCCESS);
}

- (void)sharingService:(NSSharingService *)sharingService didFailToShareItems:(NSArray *)items error:(NSError *)error {
    exit(EXIT_FAILURE);
}

@end
