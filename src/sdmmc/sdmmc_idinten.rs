#[doc = "Register `SDMMC_IDINTEN` reader"]
pub type R = crate::R<SdmmcIdintenSpec>;
#[doc = "Register `SDMMC_IDINTEN` writer"]
pub type W = crate::W<SdmmcIdintenSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
pub type DuR = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
pub type DuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
pub type CesR = crate::BitReader;
#[doc = "Field `CES` writer - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
pub type CesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NI` reader - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]
Transmit Interrupt IDINTEN\\[1\\]
Receive Interrupt"]
pub type NiR = crate::BitReader;
#[doc = "Field `NI` writer - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]
Transmit Interrupt IDINTEN\\[1\\]
Receive Interrupt"]
pub type NiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AI` reader - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[ 2\\]
Fatal Bus Error Interrupt IDINTEN\\[4\\]
DU Interrupt"]
pub type AiR = crate::BitReader;
#[doc = "Field `AI` writer - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[ 2\\]
Fatal Bus Error Interrupt IDINTEN\\[4\\]
DU Interrupt"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
    #[inline(always)]
    pub fn ces(&self) -> CesR {
        CesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]
Transmit Interrupt IDINTEN\\[1\\]
Receive Interrupt"]
    #[inline(always)]
    pub fn ni(&self) -> NiR {
        NiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[ 2\\]
Fatal Bus Error Interrupt IDINTEN\\[4\\]
DU Interrupt"]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<SdmmcIdintenSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<SdmmcIdintenSpec> {
        RiW::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FbeW<SdmmcIdintenSpec> {
        FbeW::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DuW<SdmmcIdintenSpec> {
        DuW::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CesW<SdmmcIdintenSpec> {
        CesW::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]
Transmit Interrupt IDINTEN\\[1\\]
Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ni(&mut self) -> NiW<SdmmcIdintenSpec> {
        NiW::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[ 2\\]
Fatal Bus Error Interrupt IDINTEN\\[4\\]
DU Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<SdmmcIdintenSpec> {
        AiW::new(self, 9)
    }
}
#[doc = "Internal DMAC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idinten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idinten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcIdintenSpec;
impl crate::RegisterSpec for SdmmcIdintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idinten::R`](R) reader structure"]
impl crate::Readable for SdmmcIdintenSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idinten::W`](W) writer structure"]
impl crate::Writable for SdmmcIdintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDINTEN to value 0"]
impl crate::Resettable for SdmmcIdintenSpec {
    const RESET_VALUE: u32 = 0;
}
