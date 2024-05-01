#[doc = "Register `SWREG76` reader"]
pub type R = crate::R<Swreg76Spec>;
#[doc = "Register `SWREG76` writer"]
pub type W = crate::W<Swreg76Spec>;
#[doc = "Field `H264_NUM_REF_IDX0` reader - the number of referance pic index0"]
pub type H264NumRefIdx0R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX0` writer - the number of referance pic index0"]
pub type H264NumRefIdx0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX1` reader - the number of referance pic index0"]
pub type H264NumRefIdx1R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX1` writer - the number of referance pic index0"]
pub type H264NumRefIdx1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index0"]
    #[inline(always)]
    pub fn h264_num_ref_idx0(&self) -> H264NumRefIdx0R {
        H264NumRefIdx0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index0"]
    #[inline(always)]
    pub fn h264_num_ref_idx1(&self) -> H264NumRefIdx1R {
        H264NumRefIdx1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx0(&mut self) -> H264NumRefIdx0W<Swreg76Spec> {
        H264NumRefIdx0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx1(&mut self) -> H264NumRefIdx1W<Swreg76Spec> {
        H264NumRefIdx1W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg76Spec;
impl crate::RegisterSpec for Swreg76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg76::R`](R) reader structure"]
impl crate::Readable for Swreg76Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg76::W`](W) writer structure"]
impl crate::Writable for Swreg76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG76 to value 0"]
impl crate::Resettable for Swreg76Spec {
    const RESET_VALUE: u32 = 0;
}
