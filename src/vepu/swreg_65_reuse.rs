#[doc = "Register `SWREG_65_REUSE` reader"]
pub type R = crate::R<Swreg65ReuseSpec>;
#[doc = "Register `SWREG_65_REUSE` writer"]
pub type W = crate::W<Swreg65ReuseSpec>;
#[doc = "Field `H264_CHKPT_2` reader - 2st word used for check point used in h.264\n\n2st word used for check point used in h.264"]
pub type H264Chkpt2R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_2` writer - 2st word used for check point used in h.264\n\n2st word used for check point used in h.264"]
pub type H264Chkpt2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_CHKPT_1` reader - 1st word used for check point used in h.264\n\n1st word used for check point used in h.264"]
pub type H264Chkpt1R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_1` writer - 1st word used for check point used in h.264\n\n1st word used for check point used in h.264"]
pub type H264Chkpt1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 2st word used for check point used in h.264\n\n2st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_2(&self) -> H264Chkpt2R {
        H264Chkpt2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 1st word used for check point used in h.264\n\n1st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_1(&self) -> H264Chkpt1R {
        H264Chkpt1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 2st word used for check point used in h.264\n\n2st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_2(&mut self) -> H264Chkpt2W<Swreg65ReuseSpec> {
        H264Chkpt2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 1st word used for check point used in h.264\n\n1st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_1(&mut self) -> H264Chkpt1W<Swreg65ReuseSpec> {
        H264Chkpt1W::new(self, 16)
    }
}
#[doc = "checkpoint 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_65_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_65_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg65ReuseSpec;
impl crate::RegisterSpec for Swreg65ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_65_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg65ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_65_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg65ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_65_REUSE to value 0"]
impl crate::Resettable for Swreg65ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
