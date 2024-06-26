#[doc = "Register `WR_CHAN_QOS_OVERRIDE_S0` reader"]
pub type R = crate::R<WrChanQosOverrideS0Spec>;
#[doc = "Register `WR_CHAN_QOS_OVERRIDE_S0` writer"]
pub type W = crate::W<WrChanQosOverrideS0Spec>;
#[doc = "Field `AWQOS_OVERRIDE` reader - AWQOS value override for the slave interface"]
pub type AwqosOverrideR = crate::FieldReader;
#[doc = "Field `AWQOS_OVERRIDE` writer - AWQOS value override for the slave interface"]
pub type AwqosOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AWQOS value override for the slave interface"]
    #[inline(always)]
    pub fn awqos_override(&self) -> AwqosOverrideR {
        AwqosOverrideR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AWQOS value override for the slave interface"]
    #[inline(always)]
    #[must_use]
    pub fn awqos_override(&mut self) -> AwqosOverrideW<WrChanQosOverrideS0Spec> {
        AwqosOverrideW::new(self, 0)
    }
}
#[doc = "Write Channel QoS Value Override slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_chan_qos_override_s0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_chan_qos_override_s0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrChanQosOverrideS0Spec;
impl crate::RegisterSpec for WrChanQosOverrideS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_chan_qos_override_s0::R`](R) reader structure"]
impl crate::Readable for WrChanQosOverrideS0Spec {}
#[doc = "`write(|w| ..)` method takes [`wr_chan_qos_override_s0::W`](W) writer structure"]
impl crate::Writable for WrChanQosOverrideS0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_CHAN_QOS_OVERRIDE_S0 to value 0"]
impl crate::Resettable for WrChanQosOverrideS0Spec {
    const RESET_VALUE: u32 = 0;
}
