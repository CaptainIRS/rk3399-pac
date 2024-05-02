#[doc = "Register `PHY_MUX_CTRL` reader"]
pub type R = crate::R<PhyMuxCtrlSpec>;
#[doc = "Register `PHY_MUX_CTRL` writer"]
pub type W = crate::W<PhyMuxCtrlSpec>;
#[doc = "Field `Lane_control` reader - Lane control"]
pub type LaneControlR = crate::FieldReader<u32>;
#[doc = "Field `Lane_control` writer - Lane control"]
pub type LaneControlW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SbuMuxControl {
    #[doc = "0: SBU1/SBU2 are disconnected"]
    B00 = 0,
    #[doc = "1: SBU1/SBU2 maps AUX_CH_N/AUX_CH_P"]
    B01 = 1,
    #[doc = "2: SBU1/SBU2 maps to AUX_CH_P/AUX_CH_N"]
    B10 = 2,
}
impl From<SbuMuxControl> for u8 {
    #[inline(always)]
    fn from(variant: SbuMuxControl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SbuMuxControl {
    type Ux = u8;
}
#[doc = "Field `SBU_MUX_Control` reader - "]
pub type SbuMuxControlR = crate::FieldReader<SbuMuxControl>;
impl SbuMuxControlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SbuMuxControl> {
        match self.bits {
            0 => Some(SbuMuxControl::B00),
            1 => Some(SbuMuxControl::B01),
            2 => Some(SbuMuxControl::B10),
            _ => None,
        }
    }
    #[doc = "SBU1/SBU2 are disconnected"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SbuMuxControl::B00
    }
    #[doc = "SBU1/SBU2 maps AUX_CH_N/AUX_CH_P"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SbuMuxControl::B01
    }
    #[doc = "SBU1/SBU2 maps to AUX_CH_P/AUX_CH_N"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SbuMuxControl::B10
    }
}
#[doc = "Field `SBU_MUX_Control` writer - "]
pub type SbuMuxControlW<'a, REG> = crate::FieldWriter<'a, REG, 2, SbuMuxControl>;
impl<'a, REG> SbuMuxControlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SBU1/SBU2 are disconnected"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SbuMuxControl::B00)
    }
    #[doc = "SBU1/SBU2 maps AUX_CH_N/AUX_CH_P"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SbuMuxControl::B01)
    }
    #[doc = "SBU1/SBU2 maps to AUX_CH_P/AUX_CH_N"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SbuMuxControl::B10)
    }
}
impl R {
    #[doc = "Bits 0:29 - Lane control"]
    #[inline(always)]
    pub fn lane_control(&self) -> LaneControlR {
        LaneControlR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sbu_mux_control(&self) -> SbuMuxControlR {
        SbuMuxControlR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29 - Lane control"]
    #[inline(always)]
    #[must_use]
    pub fn lane_control(&mut self) -> LaneControlW<PhyMuxCtrlSpec> {
        LaneControlW::new(self, 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn sbu_mux_control(&mut self) -> SbuMuxControlW<PhyMuxCtrlSpec> {
        SbuMuxControlW::new(self, 30)
    }
}
#[doc = "PHY MUX Ctrl Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_mux_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_mux_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyMuxCtrlSpec;
impl crate::RegisterSpec for PhyMuxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_mux_ctrl::R`](R) reader structure"]
impl crate::Readable for PhyMuxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_mux_ctrl::W`](W) writer structure"]
impl crate::Writable for PhyMuxCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_MUX_CTRL to value 0"]
impl crate::Resettable for PhyMuxCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
