#[doc = "Register `DMAC_CR1` reader"]
pub type R = crate::R<DmacCr1Spec>;
#[doc = "Field `DMAC_CR1_BITS_3` reader - The length of an i-cache line: b000-b001 = reserved b010 = 4 bytes b011 = 8 bytes b100 = 16 bytes b101 = 32 bytes b110-b111 = reserved"]
pub type DmacCr1Bits3R = crate::FieldReader;
#[doc = "Field `DMAC_CR1_BITS_1` reader - 7:4\\]
num_i-cache_lines Number of i-cache lines: b0000 = 1 i-cache line b0001 = 2 i-cache lines b0010 = 3 i-cache lines ... b1111 = 16 i-cache lines."]
pub type DmacCr1Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The length of an i-cache line: b000-b001 = reserved b010 = 4 bytes b011 = 8 bytes b100 = 16 bytes b101 = 32 bytes b110-b111 = reserved"]
    #[inline(always)]
    pub fn dmac_cr1_bits_3(&self) -> DmacCr1Bits3R {
        DmacCr1Bits3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
num_i-cache_lines Number of i-cache lines: b0000 = 1 i-cache line b0001 = 2 i-cache lines b0010 = 3 i-cache lines ... b1111 = 16 i-cache lines."]
    #[inline(always)]
    pub fn dmac_cr1_bits_1(&self) -> DmacCr1Bits1R {
        DmacCr1Bits1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCr1Spec;
impl crate::RegisterSpec for DmacCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cr1::R`](R) reader structure"]
impl crate::Readable for DmacCr1Spec {}
#[doc = "`reset()` method sets DMAC_CR1 to value 0"]
impl crate::Resettable for DmacCr1Spec {
    const RESET_VALUE: u32 = 0;
}
