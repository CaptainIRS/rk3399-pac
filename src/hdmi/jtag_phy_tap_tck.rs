#[doc = "Register `JTAG_PHY_TAP_TCK` reader"]
pub type R = crate::R<JtagPhyTapTckSpec>;
#[doc = "Register `JTAG_PHY_TAP_TCK` writer"]
pub type W = crate::W<JtagPhyTapTckSpec>;
#[doc = "Field `JTAG_TCK` reader - Configures the JTAG PHY interface pin JTAG_TCK\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tck when PHY_EXTERNAL=1."]
pub type JtagTckR = crate::BitReader;
#[doc = "Field `JTAG_TCK` writer - Configures the JTAG PHY interface pin JTAG_TCK\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tck when PHY_EXTERNAL=1."]
pub type JtagTckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the JTAG PHY interface pin JTAG_TCK\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tck when PHY_EXTERNAL=1."]
    #[inline(always)]
    pub fn jtag_tck(&self) -> JtagTckR {
        JtagTckR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the JTAG PHY interface pin JTAG_TCK\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_tck when PHY_EXTERNAL=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tck(&mut self) -> JtagTckW<JtagPhyTapTckSpec> {
        JtagTckW::new(self, 0)
    }
}
#[doc = "PHY JTAG Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_tap_tck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_tap_tck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagPhyTapTckSpec;
impl crate::RegisterSpec for JtagPhyTapTckSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jtag_phy_tap_tck::R`](R) reader structure"]
impl crate::Readable for JtagPhyTapTckSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag_phy_tap_tck::W`](W) writer structure"]
impl crate::Writable for JtagPhyTapTckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets JTAG_PHY_TAP_TCK to value 0"]
impl crate::Resettable for JtagPhyTapTckSpec {
    const RESET_VALUE: u8 = 0;
}
