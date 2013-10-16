// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(non_uppercase_statics)];

use core_foundation;
use core_foundation::array::CFArrayRef;
use core_foundation::base::AbstractCFTypeRef;
use core_foundation::base::{CFTypeID, CFTypeRef, CFWrapper};
use core_foundation::dictionary::{CFDictionaryRef, UntypedCFDictionary};
use core_foundation::number::{CFNumber, CFNumberRef};
use core_foundation::set::CFSetRef;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURLRef};
use core_graphics::base::CGFloat;

use std::cast;
use std::io;

/*
* CTFontTraits.h
*/
// actually, these are extern enums
pub type CTFontFormat = u32;
pub static kCTFontFormatUnrecognized: CTFontFormat = 0;
pub static kCTFontFormatOpenTypePostScript: CTFontFormat = 1;
pub static kCTFontFormatOpenTypeTrueType: CTFontFormat = 2;
pub static kCTFontFormatTrueType: CTFontFormat = 3;
pub static kCTFontFormatPostScript: CTFontFormat = 4;
pub static kCTFontFormatBitmap: CTFontFormat = 5;

pub static kCTFontClassMaskShift: u32 = 28;

pub type CTFontSymbolicTraits = u32;
pub static kCTFontItalicTrait: CTFontSymbolicTraits = (1 << 0);
pub static kCTFontBoldTrait: CTFontSymbolicTraits = (1 << 1);
pub static kCTFontExpandedTrait: CTFontSymbolicTraits = (1 << 5);
pub static kCTFontCondensedTrait: CTFontSymbolicTraits = (1 << 6);
pub static kCTFontMonoSpaceTrait: CTFontSymbolicTraits = (1 << 10);
pub static kCTFontVerticalTrait: CTFontSymbolicTraits = (1 << 11);
pub static kCTFontUIOptimizedTrait: CTFontSymbolicTraits = (1 << 12);
pub static kCTFontClassMaskTrait: CTFontSymbolicTraits = (15 << kCTFontClassMaskShift);

pub trait SymbolicTraitAccessors {
    fn is_italic(&self) -> bool;
    fn is_bold(&self) -> bool;
    fn is_expanded(&self) -> bool;
    fn is_condensed(&self) -> bool;
    fn is_monospace(&self) -> bool;
}

impl SymbolicTraitAccessors for CTFontSymbolicTraits {
    fn is_italic(&self) -> bool { (*self & kCTFontItalicTrait) != 0 }
    fn is_bold(&self) -> bool { (*self & kCTFontBoldTrait) != 0 }
    fn is_expanded(&self) -> bool { (*self & kCTFontExpandedTrait) != 0 }
    fn is_condensed(&self) -> bool { (*self & kCTFontCondensedTrait) != 0 }
    fn is_monospace(&self) -> bool { (*self & kCTFontMonoSpaceTrait) != 0 }
}

pub type CTFontStylisticClass = u32;
pub static kCTFontUnknownClass: CTFontStylisticClass = (0 << kCTFontClassMaskShift);
pub static kCTFontOldStyleSerifsClass: CTFontStylisticClass = (1 << kCTFontClassMaskShift);
pub static kCTFontTransitionalSerifsClass: CTFontStylisticClass = (2 << kCTFontClassMaskShift);
pub static kCTFontModernSerifsClass: CTFontStylisticClass = (3 << kCTFontClassMaskShift);
pub static kCTFontClarendonSerifsClass: CTFontStylisticClass = (4 << kCTFontClassMaskShift);
pub static kCTFontSlabSerifsClass: CTFontStylisticClass = (5 << kCTFontClassMaskShift);
pub static kCTFontFreeformSerifsClass: CTFontStylisticClass = (7 << kCTFontClassMaskShift);
pub static kCTFontSansSerifClass: CTFontStylisticClass = (8 << kCTFontClassMaskShift);
pub static kCTFontOrnamentalsClass: CTFontStylisticClass = (9 << kCTFontClassMaskShift);
pub static kCTFontScriptsClass: CTFontStylisticClass = (10 << kCTFontClassMaskShift);
pub static kCTFontSymbolicClass: CTFontStylisticClass = (12 << kCTFontClassMaskShift);

pub trait StylisticClassAccessors {
    fn is_serif(&self) -> bool;
    fn is_sans_serif(&self) -> bool;
    fn is_script(&self) -> bool;
    fn is_fantasy(&self) -> bool;
    fn is_symbols(&self) -> bool;
}

impl StylisticClassAccessors for CTFontStylisticClass {
    fn is_serif(&self) -> bool {
        let any_serif_class = kCTFontOldStyleSerifsClass 
            | kCTFontTransitionalSerifsClass
            | kCTFontModernSerifsClass
            | kCTFontClarendonSerifsClass
            | kCTFontSlabSerifsClass
            | kCTFontFreeformSerifsClass;

        return (*self & any_serif_class) != 0;
    }

    fn is_sans_serif(&self) -> bool {
        return (*self & kCTFontSansSerifClass) != 0;
    }

    fn is_script(&self) -> bool {
        return (*self & kCTFontScriptsClass) != 0;
    }

    fn is_fantasy(&self) -> bool {
        return (*self & kCTFontOrnamentalsClass) != 0;
    }

    fn is_symbols(&self) -> bool {
        return (*self & kCTFontSymbolicClass) != 0;
    }
}

pub type CTFontAttributes = UntypedCFDictionary;
pub type CTFontTraits = UntypedCFDictionary;

pub trait TraitAccessors {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits;
    fn normalized_weight(&self) -> f64;
    fn normalized_width(&self) -> f64;
    fn normalized_slant(&self) -> f64;
}

trait TraitAccessorPrivate {
    fn extract_number_for_key(&self, key: CFStringRef) -> CFNumber;
}

impl TraitAccessorPrivate for CTFontTraits {
    fn extract_number_for_key(&self, key: CFStringRef) -> CFNumber {
        let value = self.get(&key);
        CFNumber::wrap_shared(core_foundation::base::downcast::<CFNumberRef>(value))
    }

}

impl TraitAccessors for CTFontTraits {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits {
        unsafe {
            let number = self.extract_number_for_key(kCTFontSymbolicTrait);
            cast::transmute(number.to_i32())
        }
    }

    fn normalized_weight(&self) -> f64 {
        unsafe {
            let number = self.extract_number_for_key(kCTFontWeightTrait);
            cast::transmute(number.to_float())
        }
    }

    fn normalized_width(&self) -> f64 {
        unsafe {
            let number = self.extract_number_for_key(kCTFontWidthTrait);
            cast::transmute(number.to_float())
        }
    }

    fn normalized_slant(&self) -> f64 {
        unsafe {
            let number = self.extract_number_for_key(kCTFontSlantTrait);
            cast::transmute(number.to_float())
        }
    }
}

/*
* CTFontDescriptor.h
*/
pub type CTFontOrientation = u32;
pub static kCTFontDefaultOrientation: CTFontOrientation = 0;
pub static kCTFontHorizontalOrientation: CTFontOrientation = 1;
pub static kCTFontVerticalOrientation: CTFontOrientation = 2;

pub type CTFontPriority = u32;
pub static kCTFontPrioritySystem: CTFontPriority = 10000;
pub static kCTFontPriorityNetwork: CTFontPriority = 20000;
pub static kCTFontPriorityComputer: CTFontPriority = 30000;
pub static kCTFontPriorityUser: CTFontPriority = 40000;
pub static kCTFontPriorityDynamic: CTFontPriority = 50000;
pub static kCTFontPriorityProcess: CTFontPriority = 60000;

struct __CTFontDescriptor { private: () }
pub type CTFontDescriptorRef = *__CTFontDescriptor;

impl AbstractCFTypeRef for CTFontDescriptorRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    #[fixed_stack_segment]
    fn type_id(_dummy: Option<CTFontDescriptorRef>) -> CFTypeID {
        unsafe {
            CTFontDescriptorGetTypeID()
        }
    }
}

pub type CTFontDescriptor = CFWrapper<CTFontDescriptorRef, (), ()>;

pub trait CTFontDescriptorMethods {
    fn family_name(&self) -> ~str;
    fn font_name(&self) -> ~str;
    fn style_name(&self) -> ~str;
    fn display_name(&self) -> ~str;
    fn font_path(&self) -> ~str;
}

trait CTFontDescriptorMethodsPrivate {
    fn get_string_attribute(&self, attribute: CFStringRef) -> Option<~str>;
}

impl CTFontDescriptorMethodsPrivate for CTFontDescriptor {
    #[fixed_stack_segment]
    fn get_string_attribute(&self, attribute: CFStringRef) -> Option<~str> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.obj, attribute);
            if value.is_null() {
                return None;
            }

            Some(CFString::wrap_owned(core_foundation::base::downcast::<CFStringRef>(
                    value)).to_str())
        }
    }

}

impl CTFontDescriptorMethods for CTFontDescriptor {
    fn family_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
        value.expect("A font must have a non-null font family name.")
    }

    fn font_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontNameAttribute);
        value.expect("A font must have a non-null name.")
    }

    fn style_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontStyleNameAttribute);
        value.expect("A font must have a non-null style name.")
    }

    fn display_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
        value.expect("A font must have a non-null display name.")
    }

    #[fixed_stack_segment]
    fn font_path(&self) -> ~str {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.obj, kCTFontURLAttribute);
            assert!(value.is_not_null());

            CFWrapper::wrap_owned(core_foundation::base::downcast::<CFURLRef>(value)).to_str()
        }
    }
}

#[fixed_stack_segment]
pub fn new_from_attributes(attributes: &CFWrapper<CFDictionaryRef, CFStringRef, CFTypeRef>)
                        -> CTFontDescriptor {
    unsafe {
        let result: CTFontDescriptorRef =
            CTFontDescriptorCreateWithAttributes(*attributes.borrow_ref());
        CFWrapper::wrap_owned(result)
    }
}

pub fn debug_descriptor(desc: &CTFontDescriptor) {
    io::println(fmt!("family: %s", desc.family_name()));
    io::println(fmt!("name: %s", desc.font_name()));
    io::println(fmt!("style: %s", desc.style_name()));
    io::println(fmt!("display: %s", desc.display_name()));
    io::println(fmt!("path: %s", desc.font_path()));
    desc.show();
}

extern {
    /*
     * CTFontTraits.h
     */

    // font trait constants
    pub static kCTFontSymbolicTrait: CFStringRef;
    pub static kCTFontWeightTrait: CFStringRef;
    pub static kCTFontWidthTrait: CFStringRef;
    pub static kCTFontSlantTrait: CFStringRef;

    /*
     * CTFontDescriptor.h
     */

    // font attribute constants. Note that the name-related attributes
    // here are somewhat flaky. Servo creates CTFont instances and
    // then uses CTFontCopyName to get more fine-grained names.
    pub static kCTFontURLAttribute:                  CFStringRef; // value: CFURLRef
    pub static kCTFontNameAttribute:                 CFStringRef; // value: CFStringRef
    pub static kCTFontDisplayNameAttribute:          CFStringRef; // value: CFStringRef
    pub static kCTFontFamilyNameAttribute:           CFStringRef; // value: CFStringRef
    pub static kCTFontStyleNameAttribute:            CFStringRef; // value: CFStringRef
    pub static kCTFontTraitsAttribute:               CFStringRef;
    pub static kCTFontVariationAttribute:            CFStringRef;
    pub static kCTFontSizeAttribute:                 CFStringRef;
    pub static kCTFontMatrixAttribute:               CFStringRef;
    pub static kCTFontCascadeListAttribute:          CFStringRef;
    pub static kCTFontCharacterSetAttribute:         CFStringRef;
    pub static kCTFontLanguagesAttribute:            CFStringRef;
    pub static kCTFontBaselineAdjustAttribute:       CFStringRef;
    pub static kCTFontMacintoshEncodingsAttribute:   CFStringRef;
    pub static kCTFontFeaturesAttribute:             CFStringRef;
    pub static kCTFontFeatureSettingsAttribute:      CFStringRef;
    pub static kCTFontFixedAdvanceAttribute:         CFStringRef;
    pub static kCTFontOrientationAttribute:          CFStringRef;
    pub static kCTFontFormatAttribute:               CFStringRef;
    pub static kCTFontRegistrationScopeAttribute:    CFStringRef;
    pub static kCTFontPriorityAttribute:             CFStringRef;
    pub static kCTFontEnabledAttribute:              CFStringRef;

    pub fn CTFontDescriptorCopyAttribute(descriptor: CTFontDescriptorRef,
                                         attribute: CFStringRef) -> CFTypeRef;
    pub fn CTFontDescriptorCopyAttributes(descriptor: CTFontDescriptorRef) -> CFDictionaryRef;
    pub fn CTFontDescriptorCopyLocalizedAttribute(descriptor: CTFontDescriptorRef,
                                                  attribute: CFStringRef,
                                                  language: *CFStringRef) -> CFTypeRef;
    pub fn CTFontDescriptorCreateCopyWithAttributes(original: CTFontDescriptorRef, 
                                                    attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateCopyWithFeature(original: CTFontDescriptorRef,
                                                 featureTypeIdentifier: CFNumberRef,
                                                 featureSelectorIdentifier: CFNumberRef) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateCopyWithVariation(original: CTFontDescriptorRef, 
                                                   variationIdentifier: CFNumberRef,
                                                   variationValue: CGFloat) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateMatchingFontDescriptor(descriptor: CTFontDescriptorRef,
                                                        mandatoryAttributes: CFSetRef) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateWithNameAndSize(name: CFStringRef, size: CGFloat) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorGetTypeID() -> CFTypeID;
}

extern {
    pub fn CTFontDescriptorCreateMatchingFontDescriptors(descriptor: CTFontDescriptorRef,
                                                         mandatoryAttributes: CFSetRef) -> CFArrayRef;
}
