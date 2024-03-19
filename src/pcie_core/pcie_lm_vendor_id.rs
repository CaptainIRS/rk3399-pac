#[doc = "Register `PCIE_LM_VENDOR_ID` reader"]
pub type R = crate::R<PcieLmVendorIdSpec>;
#[doc = "Register `PCIE_LM_VENDOR_ID` writer"]
pub type W = crate::W<PcieLmVendorIdSpec>;
#[doc = "Field `VID` reader - Vendor ID \\[VID\\]
Vendor ID"]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `VID` writer - Vendor ID \\[VID\\]
Vendor ID"]
pub type VidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SVID` reader - Subsystem Vendor ID \\[SVID\\]
Subsystem Vendor ID"]
pub type SvidR = crate::FieldReader<u16>;
#[doc = "Field `SVID` writer - Subsystem Vendor ID \\[SVID\\]
Subsystem Vendor ID"]
pub type SvidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Vendor ID \\[VID\\]
Vendor ID"]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Subsystem Vendor ID \\[SVID\\]
Subsystem Vendor ID"]
    #[inline(always)]
    pub fn svid(&self) -> SvidR {
        SvidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Vendor ID \\[VID\\]
Vendor ID"]
    #[inline(always)]
    #[must_use]
    pub fn vid(&mut self) -> VidW<PcieLmVendorIdSpec> {
        VidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Subsystem Vendor ID \\[SVID\\]
Subsystem Vendor ID"]
    #[inline(always)]
    #[must_use]
    pub fn svid(&mut self) -> SvidW<PcieLmVendorIdSpec> {
        SvidW::new(self, 16)
    }
}
#[doc = "Vendor ID Register Subsystem Vendor ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_vendor_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_vendor_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmVendorIdSpec;
impl crate::RegisterSpec for PcieLmVendorIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_vendor_id::R`](R) reader structure"]
impl crate::Readable for PcieLmVendorIdSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_vendor_id::W`](W) writer structure"]
impl crate::Writable for PcieLmVendorIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_VENDOR_ID to value 0x17cd_17cd"]
impl crate::Resettable for PcieLmVendorIdSpec {
    const RESET_VALUE: u32 = 0x17cd_17cd;
}
