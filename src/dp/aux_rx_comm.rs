#[doc = "Register `AUX_RX_COMM` reader"]
pub type R = crate::R<AuxRxCommSpec>;
#[doc = "Register `AUX_RX_COMM` writer"]
pub type W = crate::W<AuxRxCommSpec>;
#[doc = "Field `AUX_RX_COMM` reader - AUX CH received command"]
pub type AuxRxCommR = crate::FieldReader;
#[doc = "Field `AUX_RX_COMM` writer - AUX CH received command"]
pub type AuxRxCommW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AUX CH received command"]
    #[inline(always)]
    pub fn aux_rx_comm(&self) -> AuxRxCommR {
        AuxRxCommR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AUX CH received command"]
    #[inline(always)]
    #[must_use]
    pub fn aux_rx_comm(&mut self) -> AuxRxCommW<AuxRxCommSpec> {
        AuxRxCommW::new(self, 0)
    }
}
#[doc = "DP AUX RX Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_rx_comm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_rx_comm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxRxCommSpec;
impl crate::RegisterSpec for AuxRxCommSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_rx_comm::R`](R) reader structure"]
impl crate::Readable for AuxRxCommSpec {}
#[doc = "`write(|w| ..)` method takes [`aux_rx_comm::W`](W) writer structure"]
impl crate::Writable for AuxRxCommSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets AUX_RX_COMM to value 0"]
impl crate::Resettable for AuxRxCommSpec {
    const RESET_VALUE: u32 = 0;
}
