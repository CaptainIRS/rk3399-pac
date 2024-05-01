#[doc = "Register `AES_DIN_3` reader"]
pub type R = crate::R<AesDin3Spec>;
#[doc = "Register `AES_DIN_3` writer"]
pub type W = crate::W<AesDin3Spec>;
#[doc = "Field `AES_DIN_3` reader - Specifies AES Input data \\[31:0\\]"]
pub type AesDin3R = crate::FieldReader<u32>;
#[doc = "Field `AES_DIN_3` writer - Specifies AES Input data \\[31:0\\]"]
pub type AesDin3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Input data \\[31:0\\]"]
    #[inline(always)]
    pub fn aes_din_3(&self) -> AesDin3R {
        AesDin3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES Input data \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_din_3(&mut self) -> AesDin3W<AesDin3Spec> {
        AesDin3W::new(self, 0)
    }
}
#[doc = "AES Input Data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDin3Spec;
impl crate::RegisterSpec for AesDin3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_din_3::R`](R) reader structure"]
impl crate::Readable for AesDin3Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_din_3::W`](W) writer structure"]
impl crate::Writable for AesDin3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_DIN_3 to value 0"]
impl crate::Resettable for AesDin3Spec {
    const RESET_VALUE: u32 = 0;
}
