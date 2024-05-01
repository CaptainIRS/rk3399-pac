#[doc = "Register `AES_DIN_2` reader"]
pub type R = crate::R<AesDin2Spec>;
#[doc = "Register `AES_DIN_2` writer"]
pub type W = crate::W<AesDin2Spec>;
#[doc = "Field `AES_DIN_2` reader - Specifies AES Input data \\[63:32\\]"]
pub type AesDin2R = crate::FieldReader<u32>;
#[doc = "Field `AES_DIN_2` writer - Specifies AES Input data \\[63:32\\]"]
pub type AesDin2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Input data \\[63:32\\]"]
    #[inline(always)]
    pub fn aes_din_2(&self) -> AesDin2R {
        AesDin2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES Input data \\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_din_2(&mut self) -> AesDin2W<AesDin2Spec> {
        AesDin2W::new(self, 0)
    }
}
#[doc = "AES Input Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDin2Spec;
impl crate::RegisterSpec for AesDin2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_din_2::R`](R) reader structure"]
impl crate::Readable for AesDin2Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_din_2::W`](W) writer structure"]
impl crate::Writable for AesDin2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_DIN_2 to value 0"]
impl crate::Resettable for AesDin2Spec {
    const RESET_VALUE: u32 = 0;
}
