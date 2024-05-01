#[doc = "Register `TO_CNT_CFG` reader"]
pub type R = crate::R<ToCntCfgSpec>;
#[doc = "Register `TO_CNT_CFG` writer"]
pub type W = crate::W<ToCntCfgSpec>;
#[doc = "Field `LPRX_TO_CNT` reader - lprx_to_cnt\n\nThis field configures the timeout counter that triggers a low-power\n\nreception timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles)."]
pub type LprxToCntR = crate::FieldReader<u16>;
#[doc = "Field `LPRX_TO_CNT` writer - lprx_to_cnt\n\nThis field configures the timeout counter that triggers a low-power\n\nreception timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles)."]
pub type LprxToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HSTX_TO_CNT` reader - hstx_to_cnt\n\nThis field configures the timeout counter that triggers a high-speed\n\ntransmission timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles).\n\nIf using the non-burst mode and there is no sufficient time to switch\n\nfrom HS to LP and back in the period which is from one line data\n\nfinishing to the next line sync start, the DSI link returns the LP state\n\nonce per frame, then you should configure the TO_CLK_DIVISION\n\nand hstx_to_cnt to be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one FRAME data transmission * (1 + 10%)\n\nIn burst mode, RGB pixel packets are time-compressed, leaving\n\nmore time during a scan line. Therefore, if in burst mode and there\n\nis sufficient time to switch from HS to LP and back in the period of\n\ntime from one line data finishing to the next line sync start, the DSI\n\nlink can return LP mode and back in this time interval to save\n\npower. For this, configure the TO_CLK_DIVISION and hstx_to_cnt\n\nto be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one LINE data transmission * (1 + 10%)"]
pub type HstxToCntR = crate::FieldReader<u16>;
#[doc = "Field `HSTX_TO_CNT` writer - hstx_to_cnt\n\nThis field configures the timeout counter that triggers a high-speed\n\ntransmission timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles).\n\nIf using the non-burst mode and there is no sufficient time to switch\n\nfrom HS to LP and back in the period which is from one line data\n\nfinishing to the next line sync start, the DSI link returns the LP state\n\nonce per frame, then you should configure the TO_CLK_DIVISION\n\nand hstx_to_cnt to be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one FRAME data transmission * (1 + 10%)\n\nIn burst mode, RGB pixel packets are time-compressed, leaving\n\nmore time during a scan line. Therefore, if in burst mode and there\n\nis sufficient time to switch from HS to LP and back in the period of\n\ntime from one line data finishing to the next line sync start, the DSI\n\nlink can return LP mode and back in this time interval to save\n\npower. For this, configure the TO_CLK_DIVISION and hstx_to_cnt\n\nto be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one LINE data transmission * (1 + 10%)"]
pub type HstxToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - lprx_to_cnt\n\nThis field configures the timeout counter that triggers a low-power\n\nreception timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles)."]
    #[inline(always)]
    pub fn lprx_to_cnt(&self) -> LprxToCntR {
        LprxToCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - hstx_to_cnt\n\nThis field configures the timeout counter that triggers a high-speed\n\ntransmission timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles).\n\nIf using the non-burst mode and there is no sufficient time to switch\n\nfrom HS to LP and back in the period which is from one line data\n\nfinishing to the next line sync start, the DSI link returns the LP state\n\nonce per frame, then you should configure the TO_CLK_DIVISION\n\nand hstx_to_cnt to be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one FRAME data transmission * (1 + 10%)\n\nIn burst mode, RGB pixel packets are time-compressed, leaving\n\nmore time during a scan line. Therefore, if in burst mode and there\n\nis sufficient time to switch from HS to LP and back in the period of\n\ntime from one line data finishing to the next line sync start, the DSI\n\nlink can return LP mode and back in this time interval to save\n\npower. For this, configure the TO_CLK_DIVISION and hstx_to_cnt\n\nto be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one LINE data transmission * (1 + 10%)"]
    #[inline(always)]
    pub fn hstx_to_cnt(&self) -> HstxToCntR {
        HstxToCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - lprx_to_cnt\n\nThis field configures the timeout counter that triggers a low-power\n\nreception timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles)."]
    #[inline(always)]
    #[must_use]
    pub fn lprx_to_cnt(&mut self) -> LprxToCntW<ToCntCfgSpec> {
        LprxToCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - hstx_to_cnt\n\nThis field configures the timeout counter that triggers a high-speed\n\ntransmission timeout contention detection (measured in\n\nTO_CLK_DIVISION cycles).\n\nIf using the non-burst mode and there is no sufficient time to switch\n\nfrom HS to LP and back in the period which is from one line data\n\nfinishing to the next line sync start, the DSI link returns the LP state\n\nonce per frame, then you should configure the TO_CLK_DIVISION\n\nand hstx_to_cnt to be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one FRAME data transmission * (1 + 10%)\n\nIn burst mode, RGB pixel packets are time-compressed, leaving\n\nmore time during a scan line. Therefore, if in burst mode and there\n\nis sufficient time to switch from HS to LP and back in the period of\n\ntime from one line data finishing to the next line sync start, the DSI\n\nlink can return LP mode and back in this time interval to save\n\npower. For this, configure the TO_CLK_DIVISION and hstx_to_cnt\n\nto be in accordance with:\n\nhstx_to_cnt * lanebyteclkperiod * TO_CLK_DIVISION >= the time\n\nof one LINE data transmission * (1 + 10%)"]
    #[inline(always)]
    #[must_use]
    pub fn hstx_to_cnt(&mut self) -> HstxToCntW<ToCntCfgSpec> {
        HstxToCntW::new(self, 16)
    }
}
#[doc = "Timeout Timers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to_cnt_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to_cnt_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToCntCfgSpec;
impl crate::RegisterSpec for ToCntCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to_cnt_cfg::R`](R) reader structure"]
impl crate::Readable for ToCntCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`to_cnt_cfg::W`](W) writer structure"]
impl crate::Writable for ToCntCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TO_CNT_CFG to value 0"]
impl crate::Resettable for ToCntCfgSpec {
    const RESET_VALUE: u32 = 0;
}
