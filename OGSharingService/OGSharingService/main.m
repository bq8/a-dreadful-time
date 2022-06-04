//
//  main.m
//  OGSharingService
//
//  Created by Brian Quach on 2022/06/03.
//

#import <Cocoa/Cocoa.h>
#import <Foundation/Foundation.h>

#import "AppDelegate.h"

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSApplication *application = [NSApplication sharedApplication];
        AppDelegate *appDelegate = [[AppDelegate alloc] init];
        application.delegate = appDelegate;
        [application run];
    }

    return EXIT_SUCCESS;
}
