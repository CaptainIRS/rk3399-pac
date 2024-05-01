#[doc = "Register `AES_CNT_1` reader"]
pub type R = crate::R<AesCnt1Spec>;
#[doc = "Register `AES_CNT_1` writer"]
pub type W = crate::W<AesCnt1Spec>;
#[doc = "Field `AES_CNT_1` reader - Specifies AES Input Counter \\[95:64\\]."]
pub type AesCnt1R = crate::FieldReader<u32>;
#[doc = "Field `AES_CNT_1` writer - Specifies AES Input Counter \\[95:64\\]."]
pub type AesCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Input Counter \\[95:64\\]."]
    #[inline(always)]
    pub fn aes_cnt_1(&self) -> AesCnt1R {
        AesCnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES Input Counter \\[95:64\\]."]
    #[inline(always)]
    #[must_use]
    pub fn aes_cnt_1(&mut self) -> AesCnt1W<AesCnt1Spec> {
        AesCnt1W::new(self, 0)
    }
}
#[doc = "AES Input Counter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCnt1Spec;
impl crate::RegisterSpec for AesCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_cnt_1::R`](R) reader structure"]
impl crate::Readable for AesCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_cnt_1::W`](W) writer structure"]
impl crate::Writable for AesCnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CNT_1 to value 0"]
impl crate::Resettable for AesCnt1Spec {
    const RESET_VALUE: u32 = 0;
}
