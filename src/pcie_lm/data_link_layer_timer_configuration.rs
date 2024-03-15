#[doc = "Register `DATA_LINK_LAYER_TIMER_CONFIGURATION` reader"]
pub type R = crate::R<DataLinkLayerTimerConfigurationSpec>;
#[doc = "Register `DATA_LINK_LAYER_TIMER_CONFIGURATION` writer"]
pub type W = crate::W<DataLinkLayerTimerConfigurationSpec>;
#[doc = "Field `TSRT` reader - Transmit- Side Replay Timeout Adjustment \\[TSRT\\]
Additional transmit-side replay timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal replay timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4ns (maximum = +1020 ns, minimum = -1024 ns)."]
pub type TsrtR = crate::FieldReader<u16>;
#[doc = "Field `TSRT` writer - Transmit- Side Replay Timeout Adjustment \\[TSRT\\]
Additional transmit-side replay timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal replay timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4ns (maximum = +1020 ns, minimum = -1024 ns)."]
pub type TsrtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `R9` reader - Reserved \\[R9\\]
Reserved"]
pub type R9R = crate::FieldReader;
#[doc = "Field `RSART` reader - Receive-Side ACK-NAK Replay Timeout Adjustment \\[RSART\\]
Additional receive side ACK-NAK timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal ACK-NAK timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4 ns (maximum = +1020 ns, minimum = -1024 ns)."]
pub type RsartR = crate::FieldReader<u16>;
#[doc = "Field `RSART` writer - Receive-Side ACK-NAK Replay Timeout Adjustment \\[RSART\\]
Additional receive side ACK-NAK timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal ACK-NAK timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4 ns (maximum = +1020 ns, minimum = -1024 ns)."]
pub type RsartW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `R25` reader - Reserved \\[R25\\]
Reserved"]
pub type R25R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Transmit- Side Replay Timeout Adjustment \\[TSRT\\]
Additional transmit-side replay timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal replay timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4ns (maximum = +1020 ns, minimum = -1024 ns)."]
    #[inline(always)]
    pub fn tsrt(&self) -> TsrtR {
        TsrtR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - Reserved \\[R9\\]
Reserved"]
    #[inline(always)]
    pub fn r9(&self) -> R9R {
        R9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - Receive-Side ACK-NAK Replay Timeout Adjustment \\[RSART\\]
Additional receive side ACK-NAK timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal ACK-NAK timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4 ns (maximum = +1020 ns, minimum = -1024 ns)."]
    #[inline(always)]
    pub fn rsart(&self) -> RsartR {
        RsartR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Reserved \\[R25\\]
Reserved"]
    #[inline(always)]
    pub fn r25(&self) -> R25R {
        R25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit- Side Replay Timeout Adjustment \\[TSRT\\]
Additional transmit-side replay timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal replay timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4ns (maximum = +1020 ns, minimum = -1024 ns)."]
    #[inline(always)]
    #[must_use]
    pub fn tsrt(&mut self) -> TsrtW<DataLinkLayerTimerConfigurationSpec> {
        TsrtW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Receive-Side ACK-NAK Replay Timeout Adjustment \\[RSART\\]
Additional receive side ACK-NAK timer timeout interval. This 9-bit value is added as a signed 2's complement number to the internal ACK-NAK timer timeout value computed by the core based on the PCI Express Specifications. This enables the user to make minor adjustments to the spec-defined replay timer settings. Its value is in multiples of 4 ns (maximum = +1020 ns, minimum = -1024 ns)."]
    #[inline(always)]
    #[must_use]
    pub fn rsart(&mut self) -> RsartW<DataLinkLayerTimerConfigurationSpec> {
        RsartW::new(self, 16)
    }
}
#[doc = "Data Link Layer Timer Configuration Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_link_layer_timer_configuration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_link_layer_timer_configuration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataLinkLayerTimerConfigurationSpec;
impl crate::RegisterSpec for DataLinkLayerTimerConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_link_layer_timer_configuration::R`](R) reader structure"]
impl crate::Readable for DataLinkLayerTimerConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`data_link_layer_timer_configuration::W`](W) writer structure"]
impl crate::Writable for DataLinkLayerTimerConfigurationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_LINK_LAYER_TIMER_CONFIGURATION to value 0"]
impl crate::Resettable for DataLinkLayerTimerConfigurationSpec {
    const RESET_VALUE: u32 = 0;
}
