#[doc = "Register `VERID` reader"]
pub type R = crate::R<VeridSpec>;
#[doc = "Field `VERID` reader - Version identification register; register value is hard-wired. Can\n\nbe read by firmware to support different versions of core."]
pub type VeridR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version identification register; register value is hard-wired. Can\n\nbe read by firmware to support different versions of core."]
    #[inline(always)]
    pub fn verid(&self) -> VeridR {
        VeridR::new(self.bits)
    }
}
#[doc = "Version ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VeridSpec;
impl crate::RegisterSpec for VeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid::R`](R) reader structure"]
impl crate::Readable for VeridSpec {}
#[doc = "`reset()` method sets VERID to value 0x5342_270a"]
impl crate::Resettable for VeridSpec {
    const RESET_VALUE: u32 = 0x5342_270a;
}
