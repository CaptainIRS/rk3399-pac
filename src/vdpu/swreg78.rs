#[doc = "Register `SWREG78` reader"]
pub type R = crate::R<Swreg78Spec>;
#[doc = "Register `SWREG78` writer"]
pub type W = crate::W<Swreg78Spec>;
#[doc = "Field `H264_NUM_REF_IDX4` reader - the number of referance pic index4"]
pub type H264NumRefIdx4R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX4` writer - the number of referance pic index4"]
pub type H264NumRefIdx4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX5` reader - the number of referance pic index5"]
pub type H264NumRefIdx5R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX5` writer - the number of referance pic index5"]
pub type H264NumRefIdx5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index4"]
    #[inline(always)]
    pub fn h264_num_ref_idx4(&self) -> H264NumRefIdx4R {
        H264NumRefIdx4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index5"]
    #[inline(always)]
    pub fn h264_num_ref_idx5(&self) -> H264NumRefIdx5R {
        H264NumRefIdx5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index4"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx4(&mut self) -> H264NumRefIdx4W<Swreg78Spec> {
        H264NumRefIdx4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index5"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx5(&mut self) -> H264NumRefIdx5W<Swreg78Spec> {
        H264NumRefIdx5W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg78::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg78::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg78Spec;
impl crate::RegisterSpec for Swreg78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg78::R`](R) reader structure"]
impl crate::Readable for Swreg78Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg78::W`](W) writer structure"]
impl crate::Writable for Swreg78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG78 to value 0"]
impl crate::Resettable for Swreg78Spec {
    const RESET_VALUE: u32 = 0;
}
