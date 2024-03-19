#[doc = "Register `AHB_DMA_MBLENGTH0` reader"]
pub type R = crate::R<AhbDmaMblength0Spec>;
#[doc = "Field `MBURSTLENGTH` reader - Requested burst length (mburstlength\\[7:0\\])"]
pub type MburstlengthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Requested burst length (mburstlength\\[7:0\\])"]
    #[inline(always)]
    pub fn mburstlength(&self) -> MburstlengthR {
        MburstlengthR::new(self.bits)
    }
}
#[doc = "Audio DMA Burst Length Register 0\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mblength0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaMblength0Spec;
impl crate::RegisterSpec for AhbDmaMblength0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_mblength0::R`](R) reader structure"]
impl crate::Readable for AhbDmaMblength0Spec {}
#[doc = "`reset()` method sets AHB_DMA_MBLENGTH0 to value 0"]
impl crate::Resettable for AhbDmaMblength0Spec {
    const RESET_VALUE: u8 = 0;
}
