#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Field `CR1_BITS_3` reader - The length of an i-cache line:\n\nb000-b001 = reserved\n\nb010 = 4 bytes\n\nb011 = 8 bytes\n\nb100 = 16 bytes\n\nb101 = 32 bytes\n\nb110-b111 = reserved"]
pub type Cr1Bits3R = crate::FieldReader;
#[doc = "Field `CR1_BITS_1` reader - 7:4\\]
num_i-cache_lines Number of i-cache lines:\n\nb0000 = 1 i-cache line\n\nb0001 = 2 i-cache lines\n\nb0010 = 3 i-cache lines\n\n...\n\nb1111 = 16 i-cache lines."]
pub type Cr1Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The length of an i-cache line:\n\nb000-b001 = reserved\n\nb010 = 4 bytes\n\nb011 = 8 bytes\n\nb100 = 16 bytes\n\nb101 = 32 bytes\n\nb110-b111 = reserved"]
    #[inline(always)]
    pub fn cr1_bits_3(&self) -> Cr1Bits3R {
        Cr1Bits3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
num_i-cache_lines Number of i-cache lines:\n\nb0000 = 1 i-cache line\n\nb0001 = 2 i-cache lines\n\nb0010 = 3 i-cache lines\n\n...\n\nb1111 = 16 i-cache lines."]
    #[inline(always)]
    pub fn cr1_bits_1(&self) -> Cr1Bits1R {
        Cr1Bits1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
