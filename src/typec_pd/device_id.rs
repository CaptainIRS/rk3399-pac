#[doc = "Register `DEVICE_ID` reader"]
pub type R = crate::R<DeviceIdSpec>;
#[doc = "Field `bcdDevice` reader - A unique 16-bit unsigned integer. \n\nAssigned by the Vendor to identify \n\nthe version of the TCPC."]
pub type BcdDeviceR = crate::FieldReader<u16>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - A unique 16-bit unsigned integer. \n\nAssigned by the Vendor to identify \n\nthe version of the TCPC."]
    #[inline(always)]
    pub fn bcd_device(&self) -> BcdDeviceR {
        BcdDeviceR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Device ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceIdSpec;
impl crate::RegisterSpec for DeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_id::R`](R) reader structure"]
impl crate::Readable for DeviceIdSpec {}
#[doc = "`reset()` method sets DEVICE_ID to value 0"]
impl crate::Resettable for DeviceIdSpec {
    const RESET_VALUE: u32 = 0;
}
