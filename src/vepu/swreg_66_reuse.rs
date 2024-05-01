#[doc = "Register `SWREG_66_REUSE` reader"]
pub type R = crate::R<Swreg66ReuseSpec>;
#[doc = "Register `SWREG_66_REUSE` writer"]
pub type W = crate::W<Swreg66ReuseSpec>;
#[doc = "Field `H264_CHKPT_4` reader - 4st word used for check point used in h.264\n\n4st word used for check point used in h.264"]
pub type H264Chkpt4R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_4` writer - 4st word used for check point used in h.264\n\n4st word used for check point used in h.264"]
pub type H264Chkpt4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_CHKPT_3` reader - 3st word used for check point used in h.264\n\n3st word used for check point used in h.264"]
pub type H264Chkpt3R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_3` writer - 3st word used for check point used in h.264\n\n3st word used for check point used in h.264"]
pub type H264Chkpt3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 4st word used for check point used in h.264\n\n4st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_4(&self) -> H264Chkpt4R {
        H264Chkpt4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 3st word used for check point used in h.264\n\n3st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_3(&self) -> H264Chkpt3R {
        H264Chkpt3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 4st word used for check point used in h.264\n\n4st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_4(&mut self) -> H264Chkpt4W<Swreg66ReuseSpec> {
        H264Chkpt4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 3st word used for check point used in h.264\n\n3st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_3(&mut self) -> H264Chkpt3W<Swreg66ReuseSpec> {
        H264Chkpt3W::new(self, 16)
    }
}
#[doc = "checkpoint 3 and 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_66_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_66_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg66ReuseSpec;
impl crate::RegisterSpec for Swreg66ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_66_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg66ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_66_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg66ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_66_REUSE to value 0"]
impl crate::Resettable for Swreg66ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
