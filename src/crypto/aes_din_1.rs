#[doc = "Register `AES_DIN_1` reader"]
pub type R = crate::R<AesDin1Spec>;
#[doc = "Register `AES_DIN_1` writer"]
pub type W = crate::W<AesDin1Spec>;
#[doc = "Field `AES_DIN_1` reader - Specifies AES Input data \\[95:64\\]."]
pub type AesDin1R = crate::FieldReader<u32>;
#[doc = "Field `AES_DIN_1` writer - Specifies AES Input data \\[95:64\\]."]
pub type AesDin1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Input data \\[95:64\\]."]
    #[inline(always)]
    pub fn aes_din_1(&self) -> AesDin1R {
        AesDin1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES Input data \\[95:64\\]."]
    #[inline(always)]
    #[must_use]
    pub fn aes_din_1(&mut self) -> AesDin1W<AesDin1Spec> {
        AesDin1W::new(self, 0)
    }
}
#[doc = "AES Input Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDin1Spec;
impl crate::RegisterSpec for AesDin1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_din_1::R`](R) reader structure"]
impl crate::Readable for AesDin1Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_din_1::W`](W) writer structure"]
impl crate::Writable for AesDin1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_DIN_1 to value 0"]
impl crate::Resettable for AesDin1Spec {
    const RESET_VALUE: u32 = 0;
}
