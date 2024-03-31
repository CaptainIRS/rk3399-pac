#[doc = "Register `USB3_PERF_CON1` reader"]
pub type R = crate::R<Usb3PerfCon1Spec>;
#[doc = "Register `USB3_PERF_CON1` writer"]
pub type W = crate::W<Usb3PerfCon1Spec>;
#[doc = "Field `USB3_SW_RD_LATENCY_THR` reader - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type Usb3SwRdLatencyThrR = crate::FieldReader<u16>;
#[doc = "Field `USB3_SW_RD_LATENCY_THR` writer - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type Usb3SwRdLatencyThrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    pub fn usb3_sw_rd_latency_thr(&self) -> Usb3SwRdLatencyThrR {
        Usb3SwRdLatencyThrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_rd_latency_thr(&mut self) -> Usb3SwRdLatencyThrW<Usb3PerfCon1Spec> {
        Usb3SwRdLatencyThrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usb3PerfCon1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3PerfCon1Spec;
impl crate::RegisterSpec for Usb3PerfCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_perf_con1::R`](R) reader structure"]
impl crate::Readable for Usb3PerfCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_perf_con1::W`](W) writer structure"]
impl crate::Writable for Usb3PerfCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_PERF_CON1 to value 0"]
impl crate::Resettable for Usb3PerfCon1Spec {
    const RESET_VALUE: u32 = 0;
}
