#[doc = "Register `CCI500_RD_CHAN_QOS_OVERRIDE_S1` reader"]
pub type R = crate::R<Cci500RdChanQosOverrideS1Spec>;
#[doc = "Register `CCI500_RD_CHAN_QOS_OVERRIDE_S1` writer"]
pub type W = crate::W<Cci500RdChanQosOverrideS1Spec>;
#[doc = "Field `ARQOS_OVERRIDE` reader - ARQOS value override for the slave interface"]
pub type ArqosOverrideR = crate::FieldReader;
#[doc = "Field `ARQOS_OVERRIDE` writer - ARQOS value override for the slave interface"]
pub type ArqosOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ARQOS value override for the slave interface"]
    #[inline(always)]
    pub fn arqos_override(&self) -> ArqosOverrideR {
        ArqosOverrideR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ARQOS value override for the slave interface"]
    #[inline(always)]
    #[must_use]
    pub fn arqos_override(&mut self) -> ArqosOverrideW<Cci500RdChanQosOverrideS1Spec> {
        ArqosOverrideW::new(self, 0)
    }
}
#[doc = "Read Channel QoS Value Override Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_rd_chan_qos_override_s1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_rd_chan_qos_override_s1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500RdChanQosOverrideS1Spec;
impl crate::RegisterSpec for Cci500RdChanQosOverrideS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_rd_chan_qos_override_s1::R`](R) reader structure"]
impl crate::Readable for Cci500RdChanQosOverrideS1Spec {}
#[doc = "`write(|w| ..)` method takes [`cci500_rd_chan_qos_override_s1::W`](W) writer structure"]
impl crate::Writable for Cci500RdChanQosOverrideS1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_RD_CHAN_QOS_OVERRIDE_S1 to value 0"]
impl crate::Resettable for Cci500RdChanQosOverrideS1Spec {
    const RESET_VALUE: u32 = 0;
}
