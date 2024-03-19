#[doc = "Register `SDMMC_EMMC_DDR_REG` reader"]
pub type R = crate::R<SdmmcEmmcDdrRegSpec>;
#[doc = "Register `SDMMC_EMMC_DDR_REG` writer"]
pub type W = crate::W<SdmmcEmmcDdrRegSpec>;
#[doc = "Control for start bit detection mechanism inside Mobile Storage\n\nHost Controller based on duration of start bit; each bit refers to\n\none slot. For eMMC 4.5, start bit can be:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HalfStartBit {
    #[doc = "0: Full cycle (HALF_START_BIT = 0)"]
    B0 = 0,
    #[doc = "1: Less than one full cycle (HALF_START_BIT = 1) Set HALF_START_BIT=1 for eMMC 4.5 and above; set to 0 for SD applications."]
    B1 = 1,
}
impl From<HalfStartBit> for bool {
    #[inline(always)]
    fn from(variant: HalfStartBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALF_START_BIT` reader - Control for start bit detection mechanism inside Mobile Storage\n\nHost Controller based on duration of start bit; each bit refers to\n\none slot. For eMMC 4.5, start bit can be:"]
pub type HalfStartBitR = crate::BitReader<HalfStartBit>;
impl HalfStartBitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HalfStartBit {
        match self.bits {
            false => HalfStartBit::B0,
            true => HalfStartBit::B1,
        }
    }
    #[doc = "Full cycle (HALF_START_BIT = 0)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HalfStartBit::B0
    }
    #[doc = "Less than one full cycle (HALF_START_BIT = 1) Set HALF_START_BIT=1 for eMMC 4.5 and above; set to 0 for SD applications."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HalfStartBit::B1
    }
}
#[doc = "Field `HALF_START_BIT` writer - Control for start bit detection mechanism inside Mobile Storage\n\nHost Controller based on duration of start bit; each bit refers to\n\none slot. For eMMC 4.5, start bit can be:"]
pub type HalfStartBitW<'a, REG> = crate::BitWriter<'a, REG, HalfStartBit>;
impl<'a, REG> HalfStartBitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full cycle (HALF_START_BIT = 0)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HalfStartBit::B0)
    }
    #[doc = "Less than one full cycle (HALF_START_BIT = 1) Set HALF_START_BIT=1 for eMMC 4.5 and above; set to 0 for SD applications."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HalfStartBit::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Control for start bit detection mechanism inside Mobile Storage\n\nHost Controller based on duration of start bit; each bit refers to\n\none slot. For eMMC 4.5, start bit can be:"]
    #[inline(always)]
    pub fn half_start_bit(&self) -> HalfStartBitR {
        HalfStartBitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control for start bit detection mechanism inside Mobile Storage\n\nHost Controller based on duration of start bit; each bit refers to\n\none slot. For eMMC 4.5, start bit can be:"]
    #[inline(always)]
    #[must_use]
    pub fn half_start_bit(&mut self) -> HalfStartBitW<SdmmcEmmcDdrRegSpec> {
        HalfStartBitW::new(self, 0)
    }
}
#[doc = "eMMC4.5 DDR start bit detection control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_emmc_ddr_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_emmc_ddr_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcEmmcDdrRegSpec;
impl crate::RegisterSpec for SdmmcEmmcDdrRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_emmc_ddr_reg::R`](R) reader structure"]
impl crate::Readable for SdmmcEmmcDdrRegSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_emmc_ddr_reg::W`](W) writer structure"]
impl crate::Writable for SdmmcEmmcDdrRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_EMMC_DDR_REG to value 0"]
impl crate::Resettable for SdmmcEmmcDdrRegSpec {
    const RESET_VALUE: u32 = 0;
}
