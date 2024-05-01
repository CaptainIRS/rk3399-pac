#[doc = "Register `SWREG_71_REUSE` reader"]
pub type R = crate::R<Swreg71ReuseSpec>;
#[doc = "Register `SWREG_71_REUSE` writer"]
pub type W = crate::W<Swreg71ReuseSpec>;
#[doc = "Field `H264_ERRCHKPT_4` reader - 4st word error check point used in h.264\n\n4st word error check point used in h.264"]
pub type H264Errchkpt4R = crate::FieldReader<u16>;
#[doc = "Field `H264_ERRCHKPT_4` writer - 4st word error check point used in h.264\n\n4st word error check point used in h.264"]
pub type H264Errchkpt4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_ERRCHKPT_3` reader - 3st word error check point used in h.264\n\n3st word error check point used in h.264"]
pub type H264Errchkpt3R = crate::FieldReader<u16>;
#[doc = "Field `H264_ERRCHKPT_3` writer - 3st word error check point used in h.264\n\n3st word error check point used in h.264"]
pub type H264Errchkpt3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 4st word error check point used in h.264\n\n4st word error check point used in h.264"]
    #[inline(always)]
    pub fn h264_errchkpt_4(&self) -> H264Errchkpt4R {
        H264Errchkpt4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 3st word error check point used in h.264\n\n3st word error check point used in h.264"]
    #[inline(always)]
    pub fn h264_errchkpt_3(&self) -> H264Errchkpt3R {
        H264Errchkpt3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 4st word error check point used in h.264\n\n4st word error check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_errchkpt_4(&mut self) -> H264Errchkpt4W<Swreg71ReuseSpec> {
        H264Errchkpt4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 3st word error check point used in h.264\n\n3st word error check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_errchkpt_3(&mut self) -> H264Errchkpt3W<Swreg71ReuseSpec> {
        H264Errchkpt3W::new(self, 16)
    }
}
#[doc = "checkpoint word error 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_71_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_71_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg71ReuseSpec;
impl crate::RegisterSpec for Swreg71ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_71_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg71ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_71_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg71ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_71_REUSE to value 0"]
impl crate::Resettable for Swreg71ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
