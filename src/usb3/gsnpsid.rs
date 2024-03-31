#[doc = "Register `GSNPSID` reader"]
pub type R = crate::R<GsnpsidSpec>;
#[doc = "Field `SNPSID` reader - SNPSID\n\nSNPSID\\[31:16\\]
indicates Core Identification Number. 0x5533 is\n\nASCII for U3 (DWC_usb3).\n\nSNPSID\\[15:0\\]
indicates the release number. Current Release is\n\n3.00a.\n\nSoftware uses this register to configure release-specific features\n\nin the driver."]
pub type SnpsidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SNPSID\n\nSNPSID\\[31:16\\]
indicates Core Identification Number. 0x5533 is\n\nASCII for U3 (DWC_usb3).\n\nSNPSID\\[15:0\\]
indicates the release number. Current Release is\n\n3.00a.\n\nSoftware uses this register to configure release-specific features\n\nin the driver."]
    #[inline(always)]
    pub fn snpsid(&self) -> SnpsidR {
        SnpsidR::new(self.bits)
    }
}
#[doc = "Global SNPS ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GsnpsidSpec;
impl crate::RegisterSpec for GsnpsidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsnpsid::R`](R) reader structure"]
impl crate::Readable for GsnpsidSpec {}
#[doc = "`reset()` method sets GSNPSID to value 0x5533_290a"]
impl crate::Resettable for GsnpsidSpec {
    const RESET_VALUE: u32 = 0x5533_290a;
}
