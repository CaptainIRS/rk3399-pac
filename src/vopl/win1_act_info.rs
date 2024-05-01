#[doc = "Register `WIN1_ACT_INFO` reader"]
pub type R = crate::R<Win1ActInfoSpec>;
#[doc = "Register `WIN1_ACT_INFO` writer"]
pub type W = crate::W<Win1ActInfoSpec>;
#[doc = "Field `WIN1_ACT_WIDTH` reader - Win1 active(original) window width\n\nwin_act_width = (win1 horizontial size -1)"]
pub type Win1ActWidthR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_ACT_WIDTH` writer - Win1 active(original) window width\n\nwin_act_width = (win1 horizontial size -1)"]
pub type Win1ActWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WIN1_ACT_HEIGHT` reader - Win1 active(original) window height\n\nwin_act_height = (win1 vertical size -1)"]
pub type Win1ActHeightR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_ACT_HEIGHT` writer - Win1 active(original) window height\n\nwin_act_height = (win1 vertical size -1)"]
pub type Win1ActHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Win1 active(original) window width\n\nwin_act_width = (win1 horizontial size -1)"]
    #[inline(always)]
    pub fn win1_act_width(&self) -> Win1ActWidthR {
        Win1ActWidthR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Win1 active(original) window height\n\nwin_act_height = (win1 vertical size -1)"]
    #[inline(always)]
    pub fn win1_act_height(&self) -> Win1ActHeightR {
        Win1ActHeightR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Win1 active(original) window width\n\nwin_act_width = (win1 horizontial size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win1_act_width(&mut self) -> Win1ActWidthW<Win1ActInfoSpec> {
        Win1ActWidthW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Win1 active(original) window height\n\nwin_act_height = (win1 vertical size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win1_act_height(&mut self) -> Win1ActHeightW<Win1ActInfoSpec> {
        Win1ActHeightW::new(self, 16)
    }
}
#[doc = "Win1 active window width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_act_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_act_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1ActInfoSpec;
impl crate::RegisterSpec for Win1ActInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_act_info::R`](R) reader structure"]
impl crate::Readable for Win1ActInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_act_info::W`](W) writer structure"]
impl crate::Writable for Win1ActInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_ACT_INFO to value 0x00ef_013f"]
impl crate::Resettable for Win1ActInfoSpec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
