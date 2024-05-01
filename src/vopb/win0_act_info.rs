#[doc = "Register `WIN0_ACT_INFO` reader"]
pub type R = crate::R<Win0ActInfoSpec>;
#[doc = "Register `WIN0_ACT_INFO` writer"]
pub type W = crate::W<Win0ActInfoSpec>;
#[doc = "Field `WIN0_ACT_WIDTH` reader - Win0 active(original) window width\n\nwin_act_width = (win0 horizontial size -1)"]
pub type Win0ActWidthR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_ACT_WIDTH` writer - Win0 active(original) window width\n\nwin_act_width = (win0 horizontial size -1)"]
pub type Win0ActWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WIN0_ACT_HEIGHT` reader - Win0 active(original) window height\n\nwin_act_height = (win0 vertical size -1)"]
pub type Win0ActHeightR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_ACT_HEIGHT` writer - Win0 active(original) window height\n\nwin_act_height = (win0 vertical size -1)"]
pub type Win0ActHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Win0 active(original) window width\n\nwin_act_width = (win0 horizontial size -1)"]
    #[inline(always)]
    pub fn win0_act_width(&self) -> Win0ActWidthR {
        Win0ActWidthR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Win0 active(original) window height\n\nwin_act_height = (win0 vertical size -1)"]
    #[inline(always)]
    pub fn win0_act_height(&self) -> Win0ActHeightR {
        Win0ActHeightR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Win0 active(original) window width\n\nwin_act_width = (win0 horizontial size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win0_act_width(&mut self) -> Win0ActWidthW<Win0ActInfoSpec> {
        Win0ActWidthW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Win0 active(original) window height\n\nwin_act_height = (win0 vertical size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win0_act_height(&mut self) -> Win0ActHeightW<Win0ActInfoSpec> {
        Win0ActHeightW::new(self, 16)
    }
}
#[doc = "Win0 active window width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_act_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_act_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0ActInfoSpec;
impl crate::RegisterSpec for Win0ActInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_act_info::R`](R) reader structure"]
impl crate::Readable for Win0ActInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_act_info::W`](W) writer structure"]
impl crate::Writable for Win0ActInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_ACT_INFO to value 0x00ef_013f"]
impl crate::Resettable for Win0ActInfoSpec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
