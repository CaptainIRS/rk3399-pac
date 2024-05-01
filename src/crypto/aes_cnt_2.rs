#[doc = "Register `AES_CNT_2` reader"]
pub type R = crate::R<AesCnt2Spec>;
#[doc = "Register `AES_CNT_2` writer"]
pub type W = crate::W<AesCnt2Spec>;
#[doc = "Field `AES_CNT_2` reader - Specifies AES Input Counter\\[63:32\\]"]
pub type AesCnt2R = crate::FieldReader<u32>;
#[doc = "Field `AES_CNT_2` writer - Specifies AES Input Counter\\[63:32\\]"]
pub type AesCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Input Counter\\[63:32\\]"]
    #[inline(always)]
    pub fn aes_cnt_2(&self) -> AesCnt2R {
        AesCnt2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES Input Counter\\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_cnt_2(&mut self) -> AesCnt2W<AesCnt2Spec> {
        AesCnt2W::new(self, 0)
    }
}
#[doc = "AES Input Counter 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCnt2Spec;
impl crate::RegisterSpec for AesCnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_cnt_2::R`](R) reader structure"]
impl crate::Readable for AesCnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_cnt_2::W`](W) writer structure"]
impl crate::Writable for AesCnt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CNT_2 to value 0"]
impl crate::Resettable for AesCnt2Spec {
    const RESET_VALUE: u32 = 0;
}
