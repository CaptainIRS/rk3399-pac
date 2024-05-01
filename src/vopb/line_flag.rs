#[doc = "Register `LINE_FLAG` reader"]
pub type R = crate::R<LineFlagSpec>;
#[doc = "Register `LINE_FLAG` writer"]
pub type W = crate::W<LineFlagSpec>;
#[doc = "Field `DSP_LINE_FLAG_NUM_0` reader - Line number of the Line flag interrupt 0\n\nThe display line number when the flag interrupt occur, the range is\n\n(0~ DSP_VTOTAL-1)."]
pub type DspLineFlagNum0R = crate::FieldReader<u16>;
#[doc = "Field `DSP_LINE_FLAG_NUM_0` writer - Line number of the Line flag interrupt 0\n\nThe display line number when the flag interrupt occur, the range is\n\n(0~ DSP_VTOTAL-1)."]
pub type DspLineFlagNum0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_LINE_FLAG_NUM_1` reader - Line number of the Line flag interrupt 1\n\nThe display line number when the flag interrupt 1 occur, the range\n\nis (0~ DSP_VTOTAL-1)."]
pub type DspLineFlagNum1R = crate::FieldReader<u16>;
#[doc = "Field `DSP_LINE_FLAG_NUM_1` writer - Line number of the Line flag interrupt 1\n\nThe display line number when the flag interrupt 1 occur, the range\n\nis (0~ DSP_VTOTAL-1)."]
pub type DspLineFlagNum1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Line number of the Line flag interrupt 0\n\nThe display line number when the flag interrupt occur, the range is\n\n(0~ DSP_VTOTAL-1)."]
    #[inline(always)]
    pub fn dsp_line_flag_num_0(&self) -> DspLineFlagNum0R {
        DspLineFlagNum0R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Line number of the Line flag interrupt 1\n\nThe display line number when the flag interrupt 1 occur, the range\n\nis (0~ DSP_VTOTAL-1)."]
    #[inline(always)]
    pub fn dsp_line_flag_num_1(&self) -> DspLineFlagNum1R {
        DspLineFlagNum1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Line number of the Line flag interrupt 0\n\nThe display line number when the flag interrupt occur, the range is\n\n(0~ DSP_VTOTAL-1)."]
    #[inline(always)]
    #[must_use]
    pub fn dsp_line_flag_num_0(&mut self) -> DspLineFlagNum0W<LineFlagSpec> {
        DspLineFlagNum0W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Line number of the Line flag interrupt 1\n\nThe display line number when the flag interrupt 1 occur, the range\n\nis (0~ DSP_VTOTAL-1)."]
    #[inline(always)]
    #[must_use]
    pub fn dsp_line_flag_num_1(&mut self) -> DspLineFlagNum1W<LineFlagSpec> {
        DspLineFlagNum1W::new(self, 16)
    }
}
#[doc = "Line flag config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line_flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line_flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LineFlagSpec;
impl crate::RegisterSpec for LineFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line_flag::R`](R) reader structure"]
impl crate::Readable for LineFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`line_flag::W`](W) writer structure"]
impl crate::Writable for LineFlagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINE_FLAG to value 0"]
impl crate::Resettable for LineFlagSpec {
    const RESET_VALUE: u32 = 0;
}
