#[doc = "Register `DP_SINK_STATUS` reader"]
pub type R = crate::R<DpSinkStatusSpec>;
#[doc = "Register `DP_SINK_STATUS` writer"]
pub type W = crate::W<DpSinkStatusSpec>;
#[doc = "Field `SINK_STA_0` reader - Debug register"]
pub type SinkSta0R = crate::BitReader;
#[doc = "Field `SINK_STA_0` writer - Debug register"]
pub type SinkSta0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINK_STA_1` reader - Debug register"]
pub type SinkSta1R = crate::BitReader;
#[doc = "Field `SINK_STA_1` writer - Debug register"]
pub type SinkSta1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug register"]
    #[inline(always)]
    pub fn sink_sta_0(&self) -> SinkSta0R {
        SinkSta0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug register"]
    #[inline(always)]
    pub fn sink_sta_1(&self) -> SinkSta1R {
        SinkSta1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug register"]
    #[inline(always)]
    #[must_use]
    pub fn sink_sta_0(&mut self) -> SinkSta0W<DpSinkStatusSpec> {
        SinkSta0W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug register"]
    #[inline(always)]
    #[must_use]
    pub fn sink_sta_1(&mut self) -> SinkSta1W<DpSinkStatusSpec> {
        SinkSta1W::new(self, 1)
    }
}
#[doc = "DP Sink Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_sink_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_sink_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpSinkStatusSpec;
impl crate::RegisterSpec for DpSinkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_sink_status::R`](R) reader structure"]
impl crate::Readable for DpSinkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_sink_status::W`](W) writer structure"]
impl crate::Writable for DpSinkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_SINK_STATUS to value 0"]
impl crate::Resettable for DpSinkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
