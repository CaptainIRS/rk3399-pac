#[doc = "Register `AES_CNT_3` reader"]
pub type R = crate::R<AesCnt3Spec>;
#[doc = "Register `AES_CNT_3` writer"]
pub type W = crate::W<AesCnt3Spec>;
#[doc = "Field `AES_CNT_3` reader - Specifies AES Input Counter \\[31:0\\]"]
pub type AesCnt3R = crate::FieldReader<u32>;
#[doc = "Field `AES_CNT_3` writer - Specifies AES Input Counter \\[31:0\\]"]
pub type AesCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Input Counter \\[31:0\\]"]
    #[inline(always)]
    pub fn aes_cnt_3(&self) -> AesCnt3R {
        AesCnt3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES Input Counter \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_cnt_3(&mut self) -> AesCnt3W<AesCnt3Spec> {
        AesCnt3W::new(self, 0)
    }
}
#[doc = "AES Input Counter 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCnt3Spec;
impl crate::RegisterSpec for AesCnt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_cnt_3::R`](R) reader structure"]
impl crate::Readable for AesCnt3Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_cnt_3::W`](W) writer structure"]
impl crate::Writable for AesCnt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CNT_3 to value 0"]
impl crate::Resettable for AesCnt3Spec {
    const RESET_VALUE: u32 = 0;
}
