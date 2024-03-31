#[doc = "Register `USB3PHY_STATUS0` reader"]
pub type R = crate::R<Usb3phyStatus0Spec>;
#[doc = "Register `USB3PHY_STATUS0` writer"]
pub type W = crate::W<Usb3phyStatus0Spec>;
#[doc = "Field `TYPEC_PHY0_PIPE_STATUS` reader - TypeC PHY 0 pipe status\n\n0: indicate TypeC PHY pipe ready after release\n\nTypeC PHY pipe reset."]
pub type TypecPhy0PipeStatusR = crate::BitReader;
#[doc = "Field `TYPEC_PD_PHY0_READY` reader - TypeC PD PHY 0 ready\n\n1: TypeC PD PHY ready"]
pub type TypecPdPhy0ReadyR = crate::BitReader;
#[doc = "Field `TCPC0_JTAG_XOCDMODE` reader - TCPC0 JTAG XOCDMode"]
pub type Tcpc0JtagXocdmodeR = crate::BitReader;
#[doc = "Field `TCPC0_VBUS_OVERCURRENT` reader - TCPC0 vbus overcurrent ouput\n\n1: vbus over current"]
pub type Tcpc0VbusOvercurrentR = crate::BitReader;
#[doc = "Field `TCPC0_VCONN_TO_CC1` reader - TCPC0 vconn supply to CC1\n\n1: support voconn to CC1"]
pub type Tcpc0VconnToCc1R = crate::BitReader;
#[doc = "Field `TCPC0_VCONN_TO_CC2` reader - TCPC0 supply VCONN to CC2\n\n1: TCPC0 supply VCONN to CC2"]
pub type Tcpc0VconnToCc2R = crate::BitReader;
#[doc = "Field `TCPC0_FDIS_EN` reader - TCPC0_fdis_en\n\n1: TCPC0 force discharge enable"]
pub type Tcpc0FdisEnR = crate::BitReader;
#[doc = "Field `TCPC0_BDIS_EN` reader - TCPC0_bdis_en\n\n1: TCPC0 bleed discharge enable"]
pub type Tcpc0BdisEnR = crate::BitReader;
#[doc = "Field `TCPC0_SINK_EN` reader - TCPC0_sink_en\n\n1: TCPC0 enable to sink vbus"]
pub type Tcpc0SinkEnR = crate::BitReader;
#[doc = "Field `TCPC0_VBUS_SOURCE_EN` reader - TCPC0_vbus_source_en\n\n1: select corresponding vbus source"]
pub type Tcpc0VbusSourceEnR = crate::FieldReader;
#[doc = "Field `TYPEC_PHY1_PIPE_STATUS` reader - TypeC PHY 0 pipe status\n\n0: indicate TypeC PHY pipe ready after release\n\nTypeC PHY pipe reset."]
pub type TypecPhy1PipeStatusR = crate::BitReader;
#[doc = "Field `TYPEC_PD_PHY1_READY` reader - TypeC PD PHY 1 ready\n\n1: TypeC PD PHY ready"]
pub type TypecPdPhy1ReadyR = crate::BitReader;
#[doc = "Field `TCPC1_JTAG_XOCDMODE` reader - TCPC1 JTAG XOCDMode"]
pub type Tcpc1JtagXocdmodeR = crate::BitReader;
#[doc = "Field `TCPC1_VBUS_OVERCURRENT` reader - TCPC1 vbus overcurrent ouput\n\n1: vbus over current"]
pub type Tcpc1VbusOvercurrentR = crate::BitReader;
#[doc = "Field `TCPC1_VCONN_TO_CC1` reader - TCPC1 vconn supply to CC1\n\n1: support voconn to CC1"]
pub type Tcpc1VconnToCc1R = crate::BitReader;
#[doc = "Field `TCPC1_VCONN_TO_CC2` reader - TCPC1 supply VCONN to CC2\n\n1: TCPC1 supply VCONN to CC2"]
pub type Tcpc1VconnToCc2R = crate::BitReader;
#[doc = "Field `TCPC1_FDIS_EN` reader - TCPC1_fdis_en\n\n1: TCPC1 force discharge enable"]
pub type Tcpc1FdisEnR = crate::BitReader;
#[doc = "Field `TCPC1_FDIS_EN` writer - TCPC1_fdis_en\n\n1: TCPC1 force discharge enable"]
pub type Tcpc1FdisEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCPC1_BDIS_EN` reader - TCPC1_bdis_en\n\n1: TCPC1 bleed discharge enable"]
pub type Tcpc1BdisEnR = crate::BitReader;
#[doc = "Field `TCPC1_BDIS_EN` writer - TCPC1_bdis_en\n\n1: TCPC1 bleed discharge enable"]
pub type Tcpc1BdisEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCPC1_SINK_EN` reader - TCPC1_sink_en\n\n1: TCPC1 enable to sink vbus"]
pub type Tcpc1SinkEnR = crate::BitReader;
#[doc = "Field `TCPC1_SINK_EN` writer - TCPC1_sink_en\n\n1: TCPC1 enable to sink vbus"]
pub type Tcpc1SinkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCPC1_VBUS_SOURCE_EN` reader - TCPC1_vbus_source_en\n\n1: select corresponding vbus source"]
pub type Tcpc1VbusSourceEnR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TypeC PHY 0 pipe status\n\n0: indicate TypeC PHY pipe ready after release\n\nTypeC PHY pipe reset."]
    #[inline(always)]
    pub fn typec_phy0_pipe_status(&self) -> TypecPhy0PipeStatusR {
        TypecPhy0PipeStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TypeC PD PHY 0 ready\n\n1: TypeC PD PHY ready"]
    #[inline(always)]
    pub fn typec_pd_phy0_ready(&self) -> TypecPdPhy0ReadyR {
        TypecPdPhy0ReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TCPC0 JTAG XOCDMode"]
    #[inline(always)]
    pub fn tcpc0_jtag_xocdmode(&self) -> Tcpc0JtagXocdmodeR {
        Tcpc0JtagXocdmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCPC0 vbus overcurrent ouput\n\n1: vbus over current"]
    #[inline(always)]
    pub fn tcpc0_vbus_overcurrent(&self) -> Tcpc0VbusOvercurrentR {
        Tcpc0VbusOvercurrentR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCPC0 vconn supply to CC1\n\n1: support voconn to CC1"]
    #[inline(always)]
    pub fn tcpc0_vconn_to_cc1(&self) -> Tcpc0VconnToCc1R {
        Tcpc0VconnToCc1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCPC0 supply VCONN to CC2\n\n1: TCPC0 supply VCONN to CC2"]
    #[inline(always)]
    pub fn tcpc0_vconn_to_cc2(&self) -> Tcpc0VconnToCc2R {
        Tcpc0VconnToCc2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCPC0_fdis_en\n\n1: TCPC0 force discharge enable"]
    #[inline(always)]
    pub fn tcpc0_fdis_en(&self) -> Tcpc0FdisEnR {
        Tcpc0FdisEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCPC0_bdis_en\n\n1: TCPC0 bleed discharge enable"]
    #[inline(always)]
    pub fn tcpc0_bdis_en(&self) -> Tcpc0BdisEnR {
        Tcpc0BdisEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCPC0_sink_en\n\n1: TCPC0 enable to sink vbus"]
    #[inline(always)]
    pub fn tcpc0_sink_en(&self) -> Tcpc0SinkEnR {
        Tcpc0SinkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - TCPC0_vbus_source_en\n\n1: select corresponding vbus source"]
    #[inline(always)]
    pub fn tcpc0_vbus_source_en(&self) -> Tcpc0VbusSourceEnR {
        Tcpc0VbusSourceEnR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - TypeC PHY 0 pipe status\n\n0: indicate TypeC PHY pipe ready after release\n\nTypeC PHY pipe reset."]
    #[inline(always)]
    pub fn typec_phy1_pipe_status(&self) -> TypecPhy1PipeStatusR {
        TypecPhy1PipeStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TypeC PD PHY 1 ready\n\n1: TypeC PD PHY ready"]
    #[inline(always)]
    pub fn typec_pd_phy1_ready(&self) -> TypecPdPhy1ReadyR {
        TypecPdPhy1ReadyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCPC1 JTAG XOCDMode"]
    #[inline(always)]
    pub fn tcpc1_jtag_xocdmode(&self) -> Tcpc1JtagXocdmodeR {
        Tcpc1JtagXocdmodeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TCPC1 vbus overcurrent ouput\n\n1: vbus over current"]
    #[inline(always)]
    pub fn tcpc1_vbus_overcurrent(&self) -> Tcpc1VbusOvercurrentR {
        Tcpc1VbusOvercurrentR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TCPC1 vconn supply to CC1\n\n1: support voconn to CC1"]
    #[inline(always)]
    pub fn tcpc1_vconn_to_cc1(&self) -> Tcpc1VconnToCc1R {
        Tcpc1VconnToCc1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCPC1 supply VCONN to CC2\n\n1: TCPC1 supply VCONN to CC2"]
    #[inline(always)]
    pub fn tcpc1_vconn_to_cc2(&self) -> Tcpc1VconnToCc2R {
        Tcpc1VconnToCc2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TCPC1_fdis_en\n\n1: TCPC1 force discharge enable"]
    #[inline(always)]
    pub fn tcpc1_fdis_en(&self) -> Tcpc1FdisEnR {
        Tcpc1FdisEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCPC1_bdis_en\n\n1: TCPC1 bleed discharge enable"]
    #[inline(always)]
    pub fn tcpc1_bdis_en(&self) -> Tcpc1BdisEnR {
        Tcpc1BdisEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TCPC1_sink_en\n\n1: TCPC1 enable to sink vbus"]
    #[inline(always)]
    pub fn tcpc1_sink_en(&self) -> Tcpc1SinkEnR {
        Tcpc1SinkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - TCPC1_vbus_source_en\n\n1: select corresponding vbus source"]
    #[inline(always)]
    pub fn tcpc1_vbus_source_en(&self) -> Tcpc1VbusSourceEnR {
        Tcpc1VbusSourceEnR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - TCPC1_fdis_en\n\n1: TCPC1 force discharge enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc1_fdis_en(&mut self) -> Tcpc1FdisEnW<Usb3phyStatus0Spec> {
        Tcpc1FdisEnW::new(self, 22)
    }
    #[doc = "Bit 23 - TCPC1_bdis_en\n\n1: TCPC1 bleed discharge enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc1_bdis_en(&mut self) -> Tcpc1BdisEnW<Usb3phyStatus0Spec> {
        Tcpc1BdisEnW::new(self, 23)
    }
    #[doc = "Bit 24 - TCPC1_sink_en\n\n1: TCPC1 enable to sink vbus"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc1_sink_en(&mut self) -> Tcpc1SinkEnW<Usb3phyStatus0Spec> {
        Tcpc1SinkEnW::new(self, 24)
    }
}
#[doc = "USB3PHY_STATUS0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3phyStatus0Spec;
impl crate::RegisterSpec for Usb3phyStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3phy_status0::R`](R) reader structure"]
impl crate::Readable for Usb3phyStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3phy_status0::W`](W) writer structure"]
impl crate::Writable for Usb3phyStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3PHY_STATUS0 to value 0"]
impl crate::Resettable for Usb3phyStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
