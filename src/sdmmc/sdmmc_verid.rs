#[doc = "Register `SDMMC_VERID` reader"]
pub type R = crate::R<SdmmcVeridSpec>;
#[doc = "Field `VERID` reader - Version identification register; register value is hard-wired. Can be read by firmware to support different versions of core."]
pub type VeridR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version identification register; register value is hard-wired. Can be read by firmware to support different versions of core."]
    #[inline(always)]
    pub fn verid(&self) -> VeridR {
        VeridR::new(self.bits)
    }
}
#[doc = "Version ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_verid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcVeridSpec;
impl crate::RegisterSpec for SdmmcVeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_verid::R`](R) reader structure"]
impl crate::Readable for SdmmcVeridSpec {}
#[doc = "`reset()` method sets SDMMC_VERID to value 0x5342_270a"]
impl crate::Resettable for SdmmcVeridSpec {
    const RESET_VALUE: u32 = 0x5342_270a;
}
