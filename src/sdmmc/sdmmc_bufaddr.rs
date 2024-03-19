#[doc = "Register `SDMMC_BUFADDR` reader"]
pub type R = crate::R<SdmmcBufaddrSpec>;
#[doc = "Register `SDMMC_BUFADDR` writer"]
pub type W = crate::W<SdmmcBufaddrSpec>;
#[doc = "Field `HBA` reader - Host Buffer Address Pointer. Cleared on Reset. Pointer updated\n\nby IDMAC during operation. This register points to the current\n\nData Buffer Address being accessed by the IDMAC."]
pub type HbaR = crate::FieldReader<u32>;
#[doc = "Field `HBA` writer - Host Buffer Address Pointer. Cleared on Reset. Pointer updated\n\nby IDMAC during operation. This register points to the current\n\nData Buffer Address being accessed by the IDMAC."]
pub type HbaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer. Cleared on Reset. Pointer updated\n\nby IDMAC during operation. This register points to the current\n\nData Buffer Address being accessed by the IDMAC."]
    #[inline(always)]
    pub fn hba(&self) -> HbaR {
        HbaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer. Cleared on Reset. Pointer updated\n\nby IDMAC during operation. This register points to the current\n\nData Buffer Address being accessed by the IDMAC."]
    #[inline(always)]
    #[must_use]
    pub fn hba(&mut self) -> HbaW<SdmmcBufaddrSpec> {
        HbaW::new(self, 0)
    }
}
#[doc = "Current buffer descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_bufaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_bufaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcBufaddrSpec;
impl crate::RegisterSpec for SdmmcBufaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_bufaddr::R`](R) reader structure"]
impl crate::Readable for SdmmcBufaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_bufaddr::W`](W) writer structure"]
impl crate::Writable for SdmmcBufaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_BUFADDR to value 0"]
impl crate::Resettable for SdmmcBufaddrSpec {
    const RESET_VALUE: u32 = 0;
}
