#[doc = "Register `CCI500_INTERFACE_MONITOR_CTRL` reader"]
pub type R = crate::R<Cci500InterfaceMonitorCtrlSpec>;
#[doc = "Register `CCI500_INTERFACE_MONITOR_CTRL` writer"]
pub type W = crate::W<Cci500InterfaceMonitorCtrlSpec>;
#[doc = "Field `ENABLE_INTERFACE_MONITORS` reader - 0b0 Interface Monitor counters and flags are\n\nset to 0.\n\n0b1 Enable all Interface Monitors."]
pub type EnableInterfaceMonitorsR = crate::BitReader;
#[doc = "Field `ENABLE_INTERFACE_MONITORS` writer - 0b0 Interface Monitor counters and flags are\n\nset to 0.\n\n0b1 Enable all Interface Monitors."]
pub type EnableInterfaceMonitorsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0b0 Interface Monitor counters and flags are\n\nset to 0.\n\n0b1 Enable all Interface Monitors."]
    #[inline(always)]
    pub fn enable_interface_monitors(&self) -> EnableInterfaceMonitorsR {
        EnableInterfaceMonitorsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0b0 Interface Monitor counters and flags are\n\nset to 0.\n\n0b1 Enable all Interface Monitors."]
    #[inline(always)]
    #[must_use]
    pub fn enable_interface_monitors(
        &mut self,
    ) -> EnableInterfaceMonitorsW<Cci500InterfaceMonitorCtrlSpec> {
        EnableInterfaceMonitorsW::new(self, 0)
    }
}
#[doc = "Snoop Control Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_interface_monitor_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_interface_monitor_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500InterfaceMonitorCtrlSpec;
impl crate::RegisterSpec for Cci500InterfaceMonitorCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_interface_monitor_ctrl::R`](R) reader structure"]
impl crate::Readable for Cci500InterfaceMonitorCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cci500_interface_monitor_ctrl::W`](W) writer structure"]
impl crate::Writable for Cci500InterfaceMonitorCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_INTERFACE_MONITOR_CTRL to value 0"]
impl crate::Resettable for Cci500InterfaceMonitorCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
