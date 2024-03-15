#[doc = "Register `SDMMC_DSCADDR` reader"]
pub type R = crate::R<SdmmcDscaddrSpec>;
#[doc = "Register `SDMMC_DSCADDR` writer"]
pub type W = crate::W<SdmmcDscaddrSpec>;
#[doc = "Field `HDA` reader - Host Descriptor Address Pointer. Cleared on reset. Pointer updated by IDMAC during operation. This register points to the start address of the current descriptor read by the IDMAC."]
pub type HdaR = crate::FieldReader<u32>;
#[doc = "Field `HDA` writer - Host Descriptor Address Pointer. Cleared on reset. Pointer updated by IDMAC during operation. This register points to the start address of the current descriptor read by the IDMAC."]
pub type HdaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer. Cleared on reset. Pointer updated by IDMAC during operation. This register points to the start address of the current descriptor read by the IDMAC."]
    #[inline(always)]
    pub fn hda(&self) -> HdaR {
        HdaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer. Cleared on reset. Pointer updated by IDMAC during operation. This register points to the start address of the current descriptor read by the IDMAC."]
    #[inline(always)]
    #[must_use]
    pub fn hda(&mut self) -> HdaW<SdmmcDscaddrSpec> {
        HdaW::new(self, 0)
    }
}
#[doc = "Current host descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dscaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dscaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcDscaddrSpec;
impl crate::RegisterSpec for SdmmcDscaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_dscaddr::R`](R) reader structure"]
impl crate::Readable for SdmmcDscaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_dscaddr::W`](W) writer structure"]
impl crate::Writable for SdmmcDscaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_DSCADDR to value 0"]
impl crate::Resettable for SdmmcDscaddrSpec {
    const RESET_VALUE: u32 = 0;
}
