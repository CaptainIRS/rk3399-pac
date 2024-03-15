#[doc = "Register `PLL_REG_1` reader"]
pub type R = crate::R<PllReg1Spec>;
#[doc = "Register `PLL_REG_1` writer"]
pub type W = crate::W<PllReg1Spec>;
#[doc = "Field `PLL_REF_CLK_FREQ` reader - reference CLOCK frequency: 1(default):24MHz 0:27MHz"]
pub type PllRefClkFreqR = crate::BitReader;
#[doc = "Field `PLL_REF_CLK_FREQ` writer - reference CLOCK frequency: 1(default):24MHz 0:27MHz"]
pub type PllRefClkFreqW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "FVCO:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LinkSpeed {
    #[doc = "0: Reserved"]
    B00 = 0,
    #[doc = "1: Reserved"]
    B01 = 1,
    #[doc = "2: Reserved"]
    B1x = 2,
}
impl From<LinkSpeed> for u8 {
    #[inline(always)]
    fn from(variant: LinkSpeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LinkSpeed {
    type Ux = u8;
}
#[doc = "Field `LINK_SPEED` reader - FVCO:"]
pub type LinkSpeedR = crate::FieldReader<LinkSpeed>;
impl LinkSpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LinkSpeed> {
        match self.bits {
            0 => Some(LinkSpeed::B00),
            1 => Some(LinkSpeed::B01),
            2 => Some(LinkSpeed::B1x),
            _ => None,
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == LinkSpeed::B00
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == LinkSpeed::B01
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == LinkSpeed::B1x
    }
}
impl R {
    #[doc = "Bit 0 - reference CLOCK frequency: 1(default):24MHz 0:27MHz"]
    #[inline(always)]
    pub fn pll_ref_clk_freq(&self) -> PllRefClkFreqR {
        PllRefClkFreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - FVCO:"]
    #[inline(always)]
    pub fn link_speed(&self) -> LinkSpeedR {
        LinkSpeedR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - reference CLOCK frequency: 1(default):24MHz 0:27MHz"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ref_clk_freq(&mut self) -> PllRefClkFreqW<PllReg1Spec> {
        PllRefClkFreqW::new(self, 0)
    }
}
#[doc = "Pll_control_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllReg1Spec;
impl crate::RegisterSpec for PllReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_1::R`](R) reader structure"]
impl crate::Readable for PllReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_reg_1::W`](W) writer structure"]
impl crate::Writable for PllReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets PLL_REG_1 to value 0x11"]
impl crate::Resettable for PllReg1Spec {
    const RESET_VALUE: u32 = 0x11;
}
