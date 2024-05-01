#[doc = "Register `SWREG_72_REUSE` reader"]
pub type R = crate::R<Swreg72ReuseSpec>;
#[doc = "Register `SWREG_72_REUSE` writer"]
pub type W = crate::W<Swreg72ReuseSpec>;
#[doc = "Field `H264_ERRCHKPT_6` reader - 6st word error check point used in h.264\n\n6st word error check point used in h.264"]
pub type H264Errchkpt6R = crate::FieldReader<u16>;
#[doc = "Field `H264_ERRCHKPT_6` writer - 6st word error check point used in h.264\n\n6st word error check point used in h.264"]
pub type H264Errchkpt6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_ERRCHKPT_5` reader - 5st word error check point used in h.264\n\n5st word error check point used in h.264"]
pub type H264Errchkpt5R = crate::FieldReader<u16>;
#[doc = "Field `H264_ERRCHKPT_5` writer - 5st word error check point used in h.264\n\n5st word error check point used in h.264"]
pub type H264Errchkpt5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 6st word error check point used in h.264\n\n6st word error check point used in h.264"]
    #[inline(always)]
    pub fn h264_errchkpt_6(&self) -> H264Errchkpt6R {
        H264Errchkpt6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 5st word error check point used in h.264\n\n5st word error check point used in h.264"]
    #[inline(always)]
    pub fn h264_errchkpt_5(&self) -> H264Errchkpt5R {
        H264Errchkpt5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 6st word error check point used in h.264\n\n6st word error check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_errchkpt_6(&mut self) -> H264Errchkpt6W<Swreg72ReuseSpec> {
        H264Errchkpt6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 5st word error check point used in h.264\n\n5st word error check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_errchkpt_5(&mut self) -> H264Errchkpt5W<Swreg72ReuseSpec> {
        H264Errchkpt5W::new(self, 16)
    }
}
#[doc = "checkpoint word error 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_72_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_72_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg72ReuseSpec;
impl crate::RegisterSpec for Swreg72ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_72_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg72ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_72_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg72ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_72_REUSE to value 0"]
impl crate::Resettable for Swreg72ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
