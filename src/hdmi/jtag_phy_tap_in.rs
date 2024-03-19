#[doc = "Register `JTAG_PHY_TAP_IN` reader"]
pub type R = crate::R<JtagPhyTapInSpec>;
#[doc = "Register `JTAG_PHY_TAP_IN` writer"]
pub type W = crate::W<JtagPhyTapInSpec>;
#[doc = "Field `JTAG_TDI` reader - Configures the JTAG PHY interface pin JTAG_TDI\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tdi when PHY_EXTERNAL=1."]
pub type JtagTdiR = crate::BitReader;
#[doc = "Field `JTAG_TDI` writer - Configures the JTAG PHY interface pin JTAG_TDI\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tdi when PHY_EXTERNAL=1."]
pub type JtagTdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_TMS` reader - Configures the JTAG PHY interface pin JTAG_TMS\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tms when PHY_EXTERNAL=1."]
pub type JtagTmsR = crate::BitReader;
#[doc = "Field `JTAG_TMS` writer - Configures the JTAG PHY interface pin JTAG_TMS\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tms when PHY_EXTERNAL=1."]
pub type JtagTmsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the JTAG PHY interface pin JTAG_TDI\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tdi when PHY_EXTERNAL=1."]
    #[inline(always)]
    pub fn jtag_tdi(&self) -> JtagTdiR {
        JtagTdiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the JTAG PHY interface pin JTAG_TMS\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tms when PHY_EXTERNAL=1."]
    #[inline(always)]
    pub fn jtag_tms(&self) -> JtagTmsR {
        JtagTmsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the JTAG PHY interface pin JTAG_TDI\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tdi when PHY_EXTERNAL=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tdi(&mut self) -> JtagTdiW<JtagPhyTapInSpec> {
        JtagTdiW::new(self, 0)
    }
    #[doc = "Bit 4 - Configures the JTAG PHY interface pin JTAG_TMS\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tms when PHY_EXTERNAL=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tms(&mut self) -> JtagTmsW<JtagPhyTapInSpec> {
        JtagTmsW::new(self, 4)
    }
}
#[doc = "PHY JTAG TAP In Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_tap_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_tap_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagPhyTapInSpec;
impl crate::RegisterSpec for JtagPhyTapInSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jtag_phy_tap_in::R`](R) reader structure"]
impl crate::Readable for JtagPhyTapInSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag_phy_tap_in::W`](W) writer structure"]
impl crate::Writable for JtagPhyTapInSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets JTAG_PHY_TAP_IN to value 0x10"]
impl crate::Resettable for JtagPhyTapInSpec {
    const RESET_VALUE: u8 = 0x10;
}
