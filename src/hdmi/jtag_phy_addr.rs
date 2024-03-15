#[doc = "Register `JTAG_PHY_ADDR` reader"]
pub type R = crate::R<JtagPhyAddrSpec>;
#[doc = "Register `JTAG_PHY_ADDR` writer"]
pub type W = crate::W<JtagPhyAddrSpec>;
#[doc = "Field `JTAG_ADDR` reader - Configures the JTAG PHY interface pin JTAG_ADDR\\[7:0\\]
when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_addr\\[7:0\\]
when PHY_EXTERNAL=1"]
pub type JtagAddrR = crate::FieldReader;
#[doc = "Field `JTAG_ADDR` writer - Configures the JTAG PHY interface pin JTAG_ADDR\\[7:0\\]
when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_addr\\[7:0\\]
when PHY_EXTERNAL=1"]
pub type JtagAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the JTAG PHY interface pin JTAG_ADDR\\[7:0\\]
when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_addr\\[7:0\\]
when PHY_EXTERNAL=1"]
    #[inline(always)]
    pub fn jtag_addr(&self) -> JtagAddrR {
        JtagAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the JTAG PHY interface pin JTAG_ADDR\\[7:0\\]
when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_addr\\[7:0\\]
when PHY_EXTERNAL=1"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_addr(&mut self) -> JtagAddrW<JtagPhyAddrSpec> {
        JtagAddrW::new(self, 0)
    }
}
#[doc = "Configures the JTAG PHY interface pin JTAG_ADDR\\[7:0\\]
when in internal control mode (iphy_ext_ctrl=1'b0) or iphyext_jtag_addr\\[7:0\\]
when PHY_EXTERNAL=1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagPhyAddrSpec;
impl crate::RegisterSpec for JtagPhyAddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jtag_phy_addr::R`](R) reader structure"]
impl crate::Readable for JtagPhyAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag_phy_addr::W`](W) writer structure"]
impl crate::Writable for JtagPhyAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets JTAG_PHY_ADDR to value 0"]
impl crate::Resettable for JtagPhyAddrSpec {
    const RESET_VALUE: u8 = 0;
}
