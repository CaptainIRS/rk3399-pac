#[doc = "Register `HS_WR_TO_CNT` reader"]
pub type R = crate::R<HsWrToCntSpec>;
#[doc = "Register `HS_WR_TO_CNT` writer"]
pub type W = crate::W<HsWrToCntSpec>;
#[doc = "Field `HS_WR_TO_CNT` reader - hs_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink inactive after sending a high-speed write operation. This period\n\nis measured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type HsWrToCntR = crate::FieldReader<u16>;
#[doc = "Field `HS_WR_TO_CNT` writer - hs_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink inactive after sending a high-speed write operation. This period\n\nis measured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type HsWrToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRESP_TO_MODE` reader - presp_to_mode\n\nWhen set to 1, this bit ensures that the peripheral response timeout\n\ncaused by hs_wr_to_cnt is used only once per eDPI frame, when\n\nboth the following conditions are met:\n\n■dpivsync_edpiwms has risen and fallen.\n\n■Packets originated from eDPI have been transmitted and its FIFO\n\nis empty again.\n\nIn this scenario no non-eDPI requests are sent to the D-PHY, even\n\nif there is traffic from generic, making it return to\n\nstop state. When it does so, PRESP_TO counter is activated and\n\nonly when it finishes does the controller send any other traffic that\n\nis ready."]
pub type PrespToModeR = crate::BitReader;
#[doc = "Field `PRESP_TO_MODE` writer - presp_to_mode\n\nWhen set to 1, this bit ensures that the peripheral response timeout\n\ncaused by hs_wr_to_cnt is used only once per eDPI frame, when\n\nboth the following conditions are met:\n\n■dpivsync_edpiwms has risen and fallen.\n\n■Packets originated from eDPI have been transmitted and its FIFO\n\nis empty again.\n\nIn this scenario no non-eDPI requests are sent to the D-PHY, even\n\nif there is traffic from generic, making it return to\n\nstop state. When it does so, PRESP_TO counter is activated and\n\nonly when it finishes does the controller send any other traffic that\n\nis ready."]
pub type PrespToModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - hs_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink inactive after sending a high-speed write operation. This period\n\nis measured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn hs_wr_to_cnt(&self) -> HsWrToCntR {
        HsWrToCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - presp_to_mode\n\nWhen set to 1, this bit ensures that the peripheral response timeout\n\ncaused by hs_wr_to_cnt is used only once per eDPI frame, when\n\nboth the following conditions are met:\n\n■dpivsync_edpiwms has risen and fallen.\n\n■Packets originated from eDPI have been transmitted and its FIFO\n\nis empty again.\n\nIn this scenario no non-eDPI requests are sent to the D-PHY, even\n\nif there is traffic from generic, making it return to\n\nstop state. When it does so, PRESP_TO counter is activated and\n\nonly when it finishes does the controller send any other traffic that\n\nis ready."]
    #[inline(always)]
    pub fn presp_to_mode(&self) -> PrespToModeR {
        PrespToModeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - hs_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink inactive after sending a high-speed write operation. This period\n\nis measured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn hs_wr_to_cnt(&mut self) -> HsWrToCntW<HsWrToCntSpec> {
        HsWrToCntW::new(self, 0)
    }
    #[doc = "Bit 24 - presp_to_mode\n\nWhen set to 1, this bit ensures that the peripheral response timeout\n\ncaused by hs_wr_to_cnt is used only once per eDPI frame, when\n\nboth the following conditions are met:\n\n■dpivsync_edpiwms has risen and fallen.\n\n■Packets originated from eDPI have been transmitted and its FIFO\n\nis empty again.\n\nIn this scenario no non-eDPI requests are sent to the D-PHY, even\n\nif there is traffic from generic, making it return to\n\nstop state. When it does so, PRESP_TO counter is activated and\n\nonly when it finishes does the controller send any other traffic that\n\nis ready."]
    #[inline(always)]
    #[must_use]
    pub fn presp_to_mode(&mut self) -> PrespToModeW<HsWrToCntSpec> {
        PrespToModeW::new(self, 24)
    }
}
#[doc = "Peripheral Response Timeout Definition after Hi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_wr_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_wr_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsWrToCntSpec;
impl crate::RegisterSpec for HsWrToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_wr_to_cnt::R`](R) reader structure"]
impl crate::Readable for HsWrToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`hs_wr_to_cnt::W`](W) writer structure"]
impl crate::Writable for HsWrToCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HS_WR_TO_CNT to value 0"]
impl crate::Resettable for HsWrToCntSpec {
    const RESET_VALUE: u32 = 0;
}
