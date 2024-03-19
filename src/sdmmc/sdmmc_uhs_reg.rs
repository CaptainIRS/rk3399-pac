#[doc = "Register `SDMMC_UHS_REG` reader"]
pub type R = crate::R<SdmmcUhsRegSpec>;
#[doc = "Register `SDMMC_UHS_REG` writer"]
pub type W = crate::W<SdmmcUhsRegSpec>;
#[doc = "DDR mode. Determines the voltage fed to the buffers by an\n\nexternal voltage regulator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DdrReg {
    #[doc = "0: non-DDR mode"]
    B0 = 0,
    #[doc = "1: DDR mode UHS_REG \\[16\\]
should be set for card."]
    B1 = 1,
}
impl From<DdrReg> for bool {
    #[inline(always)]
    fn from(variant: DdrReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR_REG` reader - DDR mode. Determines the voltage fed to the buffers by an\n\nexternal voltage regulator."]
pub type DdrRegR = crate::BitReader<DdrReg>;
impl DdrRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DdrReg {
        match self.bits {
            false => DdrReg::B0,
            true => DdrReg::B1,
        }
    }
    #[doc = "non-DDR mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DdrReg::B0
    }
    #[doc = "DDR mode UHS_REG \\[16\\]
should be set for card."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DdrReg::B1
    }
}
#[doc = "Field `DDR_REG` writer - DDR mode. Determines the voltage fed to the buffers by an\n\nexternal voltage regulator."]
pub type DdrRegW<'a, REG> = crate::BitWriter<'a, REG, DdrReg>;
impl<'a, REG> DdrRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-DDR mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DdrReg::B0)
    }
    #[doc = "DDR mode UHS_REG \\[16\\]
should be set for card."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DdrReg::B1)
    }
}
impl R {
    #[doc = "Bit 16 - DDR mode. Determines the voltage fed to the buffers by an\n\nexternal voltage regulator."]
    #[inline(always)]
    pub fn ddr_reg(&self) -> DdrRegR {
        DdrRegR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - DDR mode. Determines the voltage fed to the buffers by an\n\nexternal voltage regulator."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_reg(&mut self) -> DdrRegW<SdmmcUhsRegSpec> {
        DdrRegW::new(self, 16)
    }
}
#[doc = "UHS-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_uhs_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_uhs_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcUhsRegSpec;
impl crate::RegisterSpec for SdmmcUhsRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_uhs_reg::R`](R) reader structure"]
impl crate::Readable for SdmmcUhsRegSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_uhs_reg::W`](W) writer structure"]
impl crate::Writable for SdmmcUhsRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_UHS_REG to value 0"]
impl crate::Resettable for SdmmcUhsRegSpec {
    const RESET_VALUE: u32 = 0;
}
