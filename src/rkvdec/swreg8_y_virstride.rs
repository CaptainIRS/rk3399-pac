#[doc = "Register `SWREG8_Y_VIRSTRIDE` reader"]
pub type R = crate::R<Swreg8YVirstrideSpec>;
#[doc = "Register `SWREG8_Y_VIRSTRIDE` writer"]
pub type W = crate::W<Swreg8YVirstrideSpec>;
#[doc = "Field `SW_Y_VIRSTRIDE` reader - the output picture y virtual stride\n\nthe output picture y virtual stride (the unit is 128bit)\n\nthe max: (4096x1.5 +128) x 2304 = 0xdc8000\n\nwe can know the sw_uvout_base = sw_decout_base +\n\n(sw_y_virstride &lt;&lt;4)"]
pub type SwYVirstrideR = crate::FieldReader<u32>;
#[doc = "Field `SW_Y_VIRSTRIDE` writer - the output picture y virtual stride\n\nthe output picture y virtual stride (the unit is 128bit)\n\nthe max: (4096x1.5 +128) x 2304 = 0xdc8000\n\nwe can know the sw_uvout_base = sw_decout_base +\n\n(sw_y_virstride &lt;&lt;4)"]
pub type SwYVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - the output picture y virtual stride\n\nthe output picture y virtual stride (the unit is 128bit)\n\nthe max: (4096x1.5 +128) x 2304 = 0xdc8000\n\nwe can know the sw_uvout_base = sw_decout_base +\n\n(sw_y_virstride &lt;&lt;4)"]
    #[inline(always)]
    pub fn sw_y_virstride(&self) -> SwYVirstrideR {
        SwYVirstrideR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - the output picture y virtual stride\n\nthe output picture y virtual stride (the unit is 128bit)\n\nthe max: (4096x1.5 +128) x 2304 = 0xdc8000\n\nwe can know the sw_uvout_base = sw_decout_base +\n\n(sw_y_virstride &lt;&lt;4)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_y_virstride(&mut self) -> SwYVirstrideW<Swreg8YVirstrideSpec> {
        SwYVirstrideW::new(self, 0)
    }
}
#[doc = "the ouput picture y virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg8_y_virstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg8_y_virstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg8YVirstrideSpec;
impl crate::RegisterSpec for Swreg8YVirstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg8_y_virstride::R`](R) reader structure"]
impl crate::Readable for Swreg8YVirstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg8_y_virstride::W`](W) writer structure"]
impl crate::Writable for Swreg8YVirstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG8_Y_VIRSTRIDE to value 0"]
impl crate::Resettable for Swreg8YVirstrideSpec {
    const RESET_VALUE: u32 = 0;
}
