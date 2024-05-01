#[doc = "Register `WIN0_CTRL2` reader"]
pub type R = crate::R<Win0Ctrl2Spec>;
#[doc = "Register `WIN0_CTRL2` writer"]
pub type W = crate::W<Win0Ctrl2Spec>;
#[doc = "Field `WIN_RID_WIN0_YRGB` reader - axi read id of win0 yrgb channel"]
pub type WinRidWin0YrgbR = crate::FieldReader;
#[doc = "Field `WIN_RID_WIN0_YRGB` writer - axi read id of win0 yrgb channel"]
pub type WinRidWin0YrgbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIN_RID_WIN0_CBR` reader - axi read id of win0 cbr channel"]
pub type WinRidWin0CbrR = crate::FieldReader;
#[doc = "Field `WIN_RID_WIN0_CBR` writer - axi read id of win0 cbr channel"]
pub type WinRidWin0CbrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - axi read id of win0 yrgb channel"]
    #[inline(always)]
    pub fn win_rid_win0_yrgb(&self) -> WinRidWin0YrgbR {
        WinRidWin0YrgbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - axi read id of win0 cbr channel"]
    #[inline(always)]
    pub fn win_rid_win0_cbr(&self) -> WinRidWin0CbrR {
        WinRidWin0CbrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - axi read id of win0 yrgb channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_win0_yrgb(&mut self) -> WinRidWin0YrgbW<Win0Ctrl2Spec> {
        WinRidWin0YrgbW::new(self, 0)
    }
    #[doc = "Bits 4:7 - axi read id of win0 cbr channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_win0_cbr(&mut self) -> WinRidWin0CbrW<Win0Ctrl2Spec> {
        WinRidWin0CbrW::new(self, 4)
    }
}
#[doc = "Win0 ctrl register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0Ctrl2Spec;
impl crate::RegisterSpec for Win0Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_ctrl2::R`](R) reader structure"]
impl crate::Readable for Win0Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`win0_ctrl2::W`](W) writer structure"]
impl crate::Writable for Win0Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_CTRL2 to value 0x21"]
impl crate::Resettable for Win0Ctrl2Spec {
    const RESET_VALUE: u32 = 0x21;
}
