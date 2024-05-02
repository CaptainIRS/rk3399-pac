#[doc = "Register `HIST_PROP` reader"]
pub type R = crate::R<HistPropSpec>;
#[doc = "Register `HIST_PROP` writer"]
pub type W = crate::W<HistPropSpec>;
#[doc = "histogram mode, luminance is taken at ISP output\n\nbefore output formatter, RGB is taken at xtalk output\n\n7, 6: must not be used\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HistMode {
    #[doc = "5: Y (luminance) histogram 4: B histogram"]
    D5 = 5,
    #[doc = "3: G histogram"]
    D3 = 3,
    #[doc = "2: R histogram"]
    D2 = 2,
    #[doc = "1: RGB combined histogram 0: disable, no measurements"]
    D1 = 1,
}
impl From<HistMode> for u8 {
    #[inline(always)]
    fn from(variant: HistMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HistMode {
    type Ux = u8;
}
#[doc = "Field `hist_mode` reader - histogram mode, luminance is taken at ISP output\n\nbefore output formatter, RGB is taken at xtalk output\n\n7, 6: must not be used"]
pub type HistModeR = crate::FieldReader<HistMode>;
impl HistModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HistMode> {
        match self.bits {
            5 => Some(HistMode::D5),
            3 => Some(HistMode::D3),
            2 => Some(HistMode::D2),
            1 => Some(HistMode::D1),
            _ => None,
        }
    }
    #[doc = "Y (luminance) histogram 4: B histogram"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == HistMode::D5
    }
    #[doc = "G histogram"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == HistMode::D3
    }
    #[doc = "R histogram"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == HistMode::D2
    }
    #[doc = "RGB combined histogram 0: disable, no measurements"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == HistMode::D1
    }
}
#[doc = "Field `hist_mode` writer - histogram mode, luminance is taken at ISP output\n\nbefore output formatter, RGB is taken at xtalk output\n\n7, 6: must not be used"]
pub type HistModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, HistMode>;
impl<'a, REG> HistModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Y (luminance) histogram 4: B histogram"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(HistMode::D5)
    }
    #[doc = "G histogram"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(HistMode::D3)
    }
    #[doc = "R histogram"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(HistMode::D2)
    }
    #[doc = "RGB combined histogram 0: disable, no measurements"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(HistMode::D1)
    }
}
#[doc = "Field `stepsize` reader - histogram predivider, process every (stepsize)th\n\npixel, all other pixels are skipped\n\n0,1,2: not allowed\n\n3: process every third input pixel 4: process every\n\nfourth input pixel\n\n...\n\n7FH: process every 127th pixel"]
pub type StepsizeR = crate::FieldReader;
#[doc = "Field `stepsize` writer - histogram predivider, process every (stepsize)th\n\npixel, all other pixels are skipped\n\n0,1,2: not allowed\n\n3: process every third input pixel 4: process every\n\nfourth input pixel\n\n...\n\n7FH: process every 127th pixel"]
pub type StepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - histogram mode, luminance is taken at ISP output\n\nbefore output formatter, RGB is taken at xtalk output\n\n7, 6: must not be used"]
    #[inline(always)]
    pub fn hist_mode(&self) -> HistModeR {
        HistModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9 - histogram predivider, process every (stepsize)th\n\npixel, all other pixels are skipped\n\n0,1,2: not allowed\n\n3: process every third input pixel 4: process every\n\nfourth input pixel\n\n...\n\n7FH: process every 127th pixel"]
    #[inline(always)]
    pub fn stepsize(&self) -> StepsizeR {
        StepsizeR::new(((self.bits >> 3) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - histogram mode, luminance is taken at ISP output\n\nbefore output formatter, RGB is taken at xtalk output\n\n7, 6: must not be used"]
    #[inline(always)]
    #[must_use]
    pub fn hist_mode(&mut self) -> HistModeW<HistPropSpec> {
        HistModeW::new(self, 0)
    }
    #[doc = "Bits 3:9 - histogram predivider, process every (stepsize)th\n\npixel, all other pixels are skipped\n\n0,1,2: not allowed\n\n3: process every third input pixel 4: process every\n\nfourth input pixel\n\n...\n\n7FH: process every 127th pixel"]
    #[inline(always)]
    #[must_use]
    pub fn stepsize(&mut self) -> StepsizeW<HistPropSpec> {
        StepsizeW::new(self, 3)
    }
}
#[doc = "Histogram properties\n\nNote: If RGB combined mode is used, then the 3 color components are sampled one \n\n\n\nafter the other. The software has to assure that all 3 color components are inside the \n\n\n\nselected window. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_prop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_prop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistPropSpec;
impl crate::RegisterSpec for HistPropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_prop::R`](R) reader structure"]
impl crate::Readable for HistPropSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_prop::W`](W) writer structure"]
impl crate::Writable for HistPropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_PROP to value 0"]
impl crate::Resettable for HistPropSpec {
    const RESET_VALUE: u32 = 0;
}
