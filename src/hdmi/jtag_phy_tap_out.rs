#[doc = "Register `JTAG_PHY_TAP_OUT` reader"]
pub type R = crate::R<JtagPhyTapOutSpec>;
#[doc = "Field `JTAG_TDO` reader - Read JTAG PHY interface input pin JTAG_TDO when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_tdo when PHY_EXTERNAL=1 Value After Reset: 0x0"]
pub type JtagTdoR = crate::BitReader;
#[doc = "Field `JTAG_TDO_EN` reader - Read JTAG PHY interface input pin JTAG_TDO_EN when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_tdo_en when PHY_EXTERNAL=1"]
pub type JtagTdoEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read JTAG PHY interface input pin JTAG_TDO when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_tdo when PHY_EXTERNAL=1 Value After Reset: 0x0"]
    #[inline(always)]
    pub fn jtag_tdo(&self) -> JtagTdoR {
        JtagTdoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Read JTAG PHY interface input pin JTAG_TDO_EN when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_tdo_en when PHY_EXTERNAL=1"]
    #[inline(always)]
    pub fn jtag_tdo_en(&self) -> JtagTdoEnR {
        JtagTdoEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Read JTAG PHY interface input pin JTAG_TDO when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_tdo when PHY_EXTERNAL=1 Value After Reset: 0x0 jtag_phy_addr Description: PHY JTAG Address Control Register Size: 8 bits Offset: 0x3038 7:0 jtag_addr R/W Configures the JTAG PHY interface pin JTAG_ADDR\\[7:0\\]
when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_addr\\[7:0\\]
when PHY_EXTERNAL=1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_tap_out::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagPhyTapOutSpec;
impl crate::RegisterSpec for JtagPhyTapOutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jtag_phy_tap_out::R`](R) reader structure"]
impl crate::Readable for JtagPhyTapOutSpec {}
#[doc = "`reset()` method sets JTAG_PHY_TAP_OUT to value 0"]
impl crate::Resettable for JtagPhyTapOutSpec {
    const RESET_VALUE: u8 = 0;
}
