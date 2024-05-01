#[doc = "Register `SWREG77` reader"]
pub type R = crate::R<Swreg77Spec>;
#[doc = "Register `SWREG77` writer"]
pub type W = crate::W<Swreg77Spec>;
#[doc = "Field `H264_NUM_REF_IDX2` reader - the number of referance pic index2"]
pub type H264NumRefIdx2R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX2` writer - the number of referance pic index2"]
pub type H264NumRefIdx2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX3` reader - the number of referance pic index3"]
pub type H264NumRefIdx3R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX3` writer - the number of referance pic index3"]
pub type H264NumRefIdx3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index2"]
    #[inline(always)]
    pub fn h264_num_ref_idx2(&self) -> H264NumRefIdx2R {
        H264NumRefIdx2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index3"]
    #[inline(always)]
    pub fn h264_num_ref_idx3(&self) -> H264NumRefIdx3R {
        H264NumRefIdx3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx2(&mut self) -> H264NumRefIdx2W<Swreg77Spec> {
        H264NumRefIdx2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index3"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx3(&mut self) -> H264NumRefIdx3W<Swreg77Spec> {
        H264NumRefIdx3W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg77Spec;
impl crate::RegisterSpec for Swreg77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg77::R`](R) reader structure"]
impl crate::Readable for Swreg77Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg77::W`](W) writer structure"]
impl crate::Writable for Swreg77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG77 to value 0"]
impl crate::Resettable for Swreg77Spec {
    const RESET_VALUE: u32 = 0;
}
